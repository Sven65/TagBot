################# COMPILE TAGBOT #################

FROM rust:1.63.0 as build

RUN USER=root cargo new --bin tagbot
WORKDIR /tagbot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build deps for cache

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build for release

RUN rm ./target/release/deps/tagbot*
RUN cargo install --path .

################# COMPILE LUA #################

FROM debian:buster-slim as lua_builder

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

FROM lua_builder as lua_modules

RUN apt-get install -y wget git

WORKDIR /lua_mods

RUN luarocks install kikito/sandbox


################# FINAL BUILD #################

# Final build

# # TODO: Add luarocks and sandbox

FROM debian:buster-slim as final

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

COPY --from=build /lib/x86_64-linux-gnu/libm-2.31.so /lib/x86_64-linux-gnu/ 
RUN rm /lib/x86_64-linux-gnu/libm.so.6
RUN ln -s /lib/x86_64-linux-gnu/libm-2.31.so /lib/x86_64-linux-gnu/libm.so.6

## Copy tagbot binary


COPY --from=build /tagbot/target/release/tagbot ./tagbot

ENTRYPOINT './tagbot'
# CMD []
