use rlua::Lua;
use tagbot::tags::lua::{
	lua_modules::rs_lua::types::serenity::embed::{get_author_from_create_embed, TBEmbed},
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

    // assert_eq!(format!("{:?}", data.0), "CreateEmbed({\"type\": String(\"rich\")})")
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

            my_embed:set_author_icon_url("https://i.imgur.com/XDnPdoo.jpeg")
            my_embed:set_author_name("Author Name")

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
