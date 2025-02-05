use cat_loggr::log_debug;
use rlua::Lua;
use tagbot::tags::lua::{
	lua_modules::rs_lua::types::serenity::embed::{
		get_author_from_create_embed, get_footer_from_create_embed, TBEmbed,
	},
	user_require::user_require,
};

#[rustfmt::skip]
#[test]
fn create_new_embed() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            return embed:new()
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("type").unwrap(), "rich");
}

#[rustfmt::skip]
#[test]
fn should_set_author_name() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_author_name("Author Name")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    let author = get_author_from_create_embed(&data.0).unwrap_or_default();

    assert_eq!(author.0.get("name").unwrap(), "Author Name");
}

#[rustfmt::skip]
#[test]
fn set_author_icon_url() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_author_icon_url("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    let author = get_author_from_create_embed(&data.0).unwrap_or_default();

    assert_eq!(author.0.get("icon_url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");
}

#[rustfmt::skip]
#[test]
fn set_author_icon_url_preserves_author_name() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_author_name("Author Name")
            my_embed:set_author_icon_url("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    let author = get_author_from_create_embed(&data.0).unwrap_or_default();
    assert_eq!(author.0.get("name").unwrap(), "Author Name");
    assert_eq!(author.0.get("icon_url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");    
}

#[rustfmt::skip]
#[test]
fn set_colour() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")
            local colour = user_require("colour")

            local my_embed = embed:new()

            my_embed:set_colour(colour.from_rgb(12, 34, 56))

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("color").unwrap(), 795192);
}

#[rustfmt::skip]
#[test]
fn set_description() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_description("Hello, World")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("description").unwrap(), "Hello, World");
}

#[rustfmt::skip]
#[test]
fn set_image() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_image("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("image").unwrap().get("url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");
}

#[rustfmt::skip]
#[test]
fn set_thumbnail() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_thumbnail("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("thumbnail").unwrap().get("url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");
}

#[rustfmt::skip]
#[test]
fn set_timestamp() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")
            local Timestamp = user_require('timestamp')
			local time = Timestamp.new{secs = 1662796089} 

            local my_embed = embed:new()

            my_embed:set_timestamp(time)

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("timestamp").unwrap(), "2022-09-10T07:48:09Z");
}

#[rustfmt::skip]
#[test]
fn set_title() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_title("Embed title")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("title").unwrap(), "Embed title");
}

#[rustfmt::skip]
#[test]
fn set_url() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_url("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");
}

#[rustfmt::skip]
#[test]
fn set_footer_text() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_url("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    assert_eq!(data.0.0.get("url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");
}

#[rustfmt::skip]
#[test]
fn should_set_footer_text() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_footer_text("Footer text")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    let footer = get_footer_from_create_embed(&data.0).unwrap_or_default();

    assert_eq!(footer.0.get("text").unwrap(), "Footer text");
}

#[rustfmt::skip]
#[test]
fn set_footer_icon_url() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_footer_icon_url("https://i.imgur.com/XDnPdoo.jpeg")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    let footer = get_footer_from_create_embed(&data.0).unwrap_or_default();

    assert_eq!(footer.0.get("icon_url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");
}

#[rustfmt::skip]
#[test]
fn set_footer_icon_url_preserves_text() {
    tagbot::tags::lua::lua_modules::registry::init::init_modules();
    let lua = Lua::new();

    let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

    let data = lua.load(
        r#"
            local embed = user_require("embed")

            local my_embed = embed:new()

            my_embed:set_footer_icon_url("https://i.imgur.com/XDnPdoo.jpeg")
            my_embed:set_footer_text("Footer text")

            return my_embed
        "#.to_string().as_str()
    ).eval::<TBEmbed>().unwrap();

    let author = get_footer_from_create_embed(&data.0).unwrap_or_default();

    assert_eq!(author.0.get("text").unwrap(), "Footer text");
    assert_eq!(author.0.get("icon_url").unwrap(), "https://i.imgur.com/XDnPdoo.jpeg");    
}
