use rlua::Lua;
use serenity::model::Timestamp;
use tagbot::tags::lua::{
	lua_modules::rs_lua::types::serenity::timestamp::TBTimestamp, user_require::user_require,
};

fn create_timestamp() -> TBTimestamp {
	TBTimestamp::new(Timestamp::from_unix_timestamp(1662796089).unwrap())
}

#[test]
fn formats_to_string() {
	let lua = Lua::new();
	let userdata = lua.create_userdata(create_timestamp()).unwrap();
	let globals = lua.globals();
	globals.set("timestamp", userdata).unwrap();

	let data = lua
		.load(
			r#"
			return tostring(timestamp)
		"#,
		)
		.eval::<String>()
		.unwrap();

	assert_eq!(data, "2022-09-10T07:48:09Z")
}

#[test]
fn can_create_timestamp() {
	tagbot::tags::lua::lua_modules::registry::init::init_modules();

	let lua = Lua::new();
	let globals = lua.globals();

	let lua_user_require = lua.create_function(user_require).unwrap();

	globals.set("user_require", lua_user_require).unwrap();

	let data = lua
		.load(
			r#"
			local Timestamp = user_require('timestamp')
			local time = Timestamp.new{secs = 1662796089}
			return tostring(time)
		"#,
		)
		.eval::<String>()
		.unwrap();

	assert_eq!(data, "2022-09-10T07:48:09Z")
}
