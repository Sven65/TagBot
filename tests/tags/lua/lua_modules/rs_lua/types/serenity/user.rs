use rlua::{Lua};
use serenity::model::user::User;
use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::user::TBUser;
use test_case::test_case;

fn create_user() -> TBUser {
	TBUser::new(User::default())
}

#[test_case("id", "210" ; "Gets the correct ID")]
#[test_case("name", "test" ; "Gets the correct name")]
#[test_case("discriminator", "1432" ; "Gets the correct discriminator")]
#[test_case("tag", "test#1432" ; "Gets the correct tag")]
fn get_str(param: &str, expected: &str) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_user()).unwrap();
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

#[test_case("bot", true ; "Gets that the user is a bot")]
fn get_bool(param: &str, expected: bool) {
	Lua::new().context(|lua| {
		let userdata = lua.create_userdata(create_user()).unwrap();
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