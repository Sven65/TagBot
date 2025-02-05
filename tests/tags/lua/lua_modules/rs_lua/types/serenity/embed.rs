use rlua::Lua;
use tagbot::tags::lua::{
	lua_modules::rs_lua::types::serenity::embed::TBEmbed, user_require::user_require,
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

    assert_eq!(format!("{:?}", data.0), "CreateEmbed({\"type\": String(\"rich\")})")
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

    assert_eq!(format!("{:?}", data.0), "CreateEmbed({\"type\": String(\"rich\"), \"author\": Object {\"name\": String(\"Author Name\")}})")
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

    println!("Embed test 1 {:#?}", data.0);


    assert_eq!(format!("{:?}", data.0), "CreateEmbed({\"author\": Object {\"icon_url\": String(\"https://i.imgur.com/XDnPdoo.jpeg\")}, \"type\": String(\"rich\")})")
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

    println!("Embed test 2 {:#?}", data.0);

    assert_eq!(format!("{:?}", data.0), "CreateEmbed({\"type\": String(\"rich\")})")
}
