# syntax=docker/dockerfile:1.3-labs

################# COMPILE TAGBOT #################

FROM --platform="linux/amd64" rust:1.81.0-bullseye as build

# Capture deps
# COPY Cargo.toml Cargo.lock /app/
#COPY ./tagbot-macros/Cargo.toml ./tagbot-macros/Cargo.lock /app/tagbot-macros/

# We create a new lib and then use our own Cargo.toml

RUN cargo new --lib /app/
COPY ./Cargo.toml /app/
COPY ./Cargo.lock /app/

# We do the same for macros
RUN cargo new /app/tagbot-macros
COPY tagbot-macros/Cargo.toml /app/tagbot-macros/
COPY tagbot-macros/Cargo.lock /app/tagbot-macros/

COPY ./src /app/src
COPY ./tagbot-macros /app/tagbot-macros

# This step compiles only our dependencies and saves them in a layer. This is the most impactful time savings
# Note the use of --mount=type=cache. On subsequent runs, we'll have the crates already downloaded
WORKDIR /app/
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo fetch

# A bit of magic here!
# * We're mounting that cache again to use during the build, otherwise it's not present and we'll have to download those again - bad!
# * EOF syntax is neat but not without its drawbacks. We need to `set -e`, otherwise a failing command is going to continue on
# * Rust here is a bit fiddly, so we'll touch the files (even though we copied over them) to force a new build
RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  # update timestamps to force a new build
  touch /app/tagbot-macros/src/lib.rs /app/src/main.rs
  cargo build --release
EOF

CMD ["/app/target/release/tagbot"]

################# COMPILE LUA #################

FROM --platform="linux/amd64" debian:bullseye-slim as lua_builder

WORKDIR /lua-build

RUN apt update
RUN apt install -y build-essential libreadline-dev unzip

ADD http://www.lua.org/ftp/lua-5.4.0.tar.gz ./

RUN tar -zxf lua-5.4.0.tar.gz

WORKDIR ./lua-5.4.0

RUN make linux test
RUN make install

## Install luarocks

WORKDIR /luarocks

ADD https://luarocks.org/releases/luarocks-3.9.1.tar.gz ./
RUN tar zxpf luarocks-3.9.1.tar.gz
WORKDIR ./luarocks-3.9.1

RUN ./configure && make && make install

RUN rm -rf ./luarocks-3.9.1

################# INSTALL LUAROCKS MODULES #################

FROM --platform="linux/amd64" lua_builder as lua_modules

RUN apt-get install -y wget git

WORKDIR /lua_mods

RUN luarocks install kikito/sandbox


################# FINAL BUILD #################

# Final build

# # TODO: Add luarocks and sandbox

# FROM debian:buster-slim as final
FROM --platform="linux/amd64" debian:bullseye as final

WORKDIR /home


## Copy lua


### Copy bin
COPY --from=lua_builder /usr/local/bin/lua /usr/local/bin/lua
COPY --from=lua_builder /usr/local/bin/luac /usr/local/bin/luac

### Copy include
COPY --from=lua_builder /usr/local/include/lauxlib.h /usr/local/include/
COPY --from=lua_builder /usr/local/include/lua* /usr/local/include/

### Copy lib

COPY --from=lua_builder /usr/local/lib/liblua.a /usr/local/lib/
COPY --from=lua_builder /usr/local/lib/lua/ /usr/local/lib/lua/

## Copy luarocks

### Copy bin

COPY --from=lua_builder /usr/local/bin/luarocks /usr/local/bin/
COPY --from=lua_builder /usr/local/bin/luarocks-admin /usr/local/bin/

### Copy etc

COPY --from=lua_builder /usr/local/etc/luarocks/ /usr/local/etc/luarocks/

### Copy share

COPY --from=lua_builder /usr/local/share/lua/ /usr/local/share/lua/

### Copy modules

COPY --from=lua_modules /usr/local/lib/luarocks/ /usr/local/lib/luarocks/
COPY --from=lua_modules /usr/local/share/lua/ /usr/local/share/lua/


## Copy libm bs lmao

# COPY --from=build /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/ 
# RUN rm /lib/x86_64-linux-gnu/libm.so.6
# RUN ln -s /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6

## Copy tagbot binary


COPY --from=build /app/target/release/tagbot ./tagbot
COPY ./data ./data

ENTRYPOINT './tagbot'
# CMD []
