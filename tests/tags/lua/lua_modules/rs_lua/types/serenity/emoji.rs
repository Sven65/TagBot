use crate::util::creators::create_emoji;
use rlua::Lua;
use test_case::test_case;

#[rustfmt::skip]
#[test_case("animated", false ; "Gets the right value for if emoji is animated")]
#[test_case("available", true ; "Gets the right value for if emoji is available")]
#[test_case("managed", false ; "Gets the right value for if emoji is managed")]
#[test_case("require_colons", true ; "Gets the right value for if emoji needs colons")]
fn get_bool(param: &str, expected: bool) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_emoji()).unwrap();
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
#[test_case("name", "funny_emoji" ; "Gets the right name")]
fn get_str(param: &str, expected: &str) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_emoji()).unwrap();
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
#[test_case("user", true ; "Gets the creating user correctly")]
fn get_is_nil(param: &str, expected: bool) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_emoji()).unwrap();
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
#[test_case("roles", 0 ; "Gets the right amount of roles")]
fn get_vec_length(param: &str, expected: i64) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_emoji()).unwrap();
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

#[rustfmt::skip]
#[test_case("id", "1028239715465433118" ; "Gets the right emoji id")]
fn get_tostring(param: &str, expected: &str) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_emoji()).unwrap();
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
