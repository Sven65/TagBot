use rlua::Lua;
use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::channel_category::TBChannelCategory;
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
