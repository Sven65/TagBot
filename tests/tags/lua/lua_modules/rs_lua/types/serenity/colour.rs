use rlua::Lua;
use tagbot::tags::lua::{
	lua_modules::rs_lua::types::serenity::colour::TBColour, user_require::user_require,
};

#[rustfmt::skip]

#[test]
fn should_create_color() {
	tagbot::tags::lua::lua_modules::registry::init::init_modules();

	Lua::new().context(|lua| {
		let globals = lua.globals();

		let lua_user_require = lua.create_function(user_require).unwrap();

		globals.set("user_require", lua_user_require).unwrap();

		let data = lua.load(
			r#"
			
				local colour = user_require("colour")

				return colour.from_rgb(12, 34, 56)
			"#.to_string().as_str(),
		).eval::<TBColour>().unwrap();

		assert_eq!(data.0.hex(), "0C2238")
	})
}
