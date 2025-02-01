use rlua::Lua;
use test_case::test_case;

use crate::util::creators::create_guild_channel;

#[rustfmt::skip]
#[test_case("id", "172382467385196544" ; "Gets the correct id")]
fn get_tostring(param: &str, expected: &str) {
	let lua = Lua::new();
	let userdata = lua.create_userdata(create_guild_channel()).unwrap();
	let globals = lua.globals();
	globals.set("userdata", userdata).unwrap();

	let data = lua.load(
		format!(r#"
			return tostring(userdata.{})
		"#, param).as_str(),
	).eval::<String>().unwrap();

	assert_eq!(data, expected)
}

#[rustfmt::skip]
#[test_case("category", true ; "Gets that the channel has no category")]
fn get_is_nil(param: &str, expected: bool) {
	let lua = Lua::new();
	let userdata = lua.create_userdata(create_guild_channel()).unwrap();
	let globals = lua.globals();
	globals.set("userdata", userdata).unwrap();

	let data = lua.load(
		format!(r#"
			return userdata.{} == Nil
		"#, param).as_str(),
	).eval::<bool>().unwrap();

	assert_eq!(data, expected)
}

#[rustfmt::skip]
#[test_case("is_nsfw", false ; "Gets that the channel is not nsfw")]
fn get_bool(param: &str, expected: bool) {
	let lua = Lua::new();
	let userdata = lua.create_userdata(create_guild_channel()).unwrap();
	let globals = lua.globals();
	globals.set("userdata", userdata).unwrap();

	let data = lua.load(
		format!(r#"
			return userdata.{}
		"#, param).as_str(),
	).eval::<bool>().unwrap();

	assert_eq!(data, expected)
}

#[rustfmt::skip]
#[test_case("position", 0 ; "Gets the right channel position")]
fn get_i64(param: &str, expected: i64) {
	let lua = Lua::new();
	let userdata = lua.create_userdata(create_guild_channel()).unwrap();
	let globals = lua.globals();
	globals.set("userdata", userdata).unwrap();

	let data = lua.load(
		format!(r#"
			return userdata.{}
		"#, param).as_str(),
	).eval::<i64>().unwrap();

	assert_eq!(data, expected)
}
