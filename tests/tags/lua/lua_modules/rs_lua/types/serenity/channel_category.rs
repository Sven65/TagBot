use rlua::Lua;
use test_case::test_case;

use crate::util::creators::create_channel_category;

#[rustfmt::skip]
#[test_case("kind", "category" ; "Gets the right kind")]
fn get_str(param: &str, expected: &str) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_channel_category()).unwrap();
		let globals = lua.globals();
		globals.set("userdata", userdata).unwrap();
		
		let data = lua.load(
			format!(r#"
				return userdata.{}
			"#, param).as_str(),
		).eval::<String>().unwrap();
		
		assert_eq!(data, expected)
	})
}

#[rustfmt::skip]
#[test_case("nsfw", false ; "Gets the right nsfw status")]
fn get_bool(param: &str, expected: bool) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_channel_category()).unwrap();
		let globals = lua.globals();
		globals.set("userdata", userdata).unwrap();

		let data = lua.load(
			format!(r#"
				return userdata.{}
			"#, param).as_str(),
		).eval::<bool>().unwrap();

		assert_eq!(data, expected)
	})
}

#[rustfmt::skip]
#[test_case("position", 1 ; "Gets the correct position")]
fn get_i64(param: &str, expected: i64) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_channel_category()).unwrap();
		let globals = lua.globals();
		globals.set("userdata", userdata).unwrap();

		let data = lua.load(
			format!(r#"
				return userdata.{}
			"#, param).as_str(),
		).eval::<i64>().unwrap();

		assert_eq!(data, expected)
	})
}

#[rustfmt::skip]
#[test_case("parent_id", true ; "Gets that parent id is nil")]
fn get_is_nil(param: &str, expected: bool) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_channel_category()).unwrap();
		let globals = lua.globals();
		globals.set("userdata", userdata).unwrap();

		let data = lua.load(
			format!(r#"
				return userdata.{} == Nil
			"#, param).as_str(),
		).eval::<bool>().unwrap();

		assert_eq!(data, expected)
	})
}

#[rustfmt::skip]
#[test_case("guild_id", "355959445907570698" ; "Gets the correct guild id")]
#[test_case("id", "172382467385196544" ; "Gets the correct id")]
fn get_tostring(param: &str, expected: &str) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_channel_category()).unwrap();
		let globals = lua.globals();
		globals.set("userdata", userdata).unwrap();

		let data = lua.load(
			format!(r#"
				return tostring(userdata.{})
			"#, param).as_str(),
		).eval::<String>().unwrap();

		assert_eq!(data, expected)
	})
}

#[rustfmt::skip]
#[ignore = "Todo: Not yet implemented"]
#[test_case("permission_overwrites", 0 ; "Gets the correct amount of permission overwrites")]
fn get_vec_length(param: &str, expected: i64) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_channel_category()).unwrap();
		let globals = lua.globals();
		globals.set("userdata", userdata).unwrap();

		let data = lua.load(
			format!(r#"
				return #userdata.{}
			"#, param).as_str(),
		).eval::<i64>().unwrap();

		assert_eq!(data, expected)
	})
}
