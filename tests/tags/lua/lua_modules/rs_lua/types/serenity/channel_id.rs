use rlua::Lua;
use serenity::model::prelude::ChannelId;
use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::channel_id::TBChannelId;

use crate::util::creators::create_http;

#[rustfmt::skip]
#[test]
fn channel_id_to_string() {
	let channel_id = ChannelId::new(172382467385196544);
	let tb_channel_id = TBChannelId::new(channel_id, create_http());

	let lua = Lua::new();
	let userdata = lua.create_userdata(tb_channel_id).unwrap();
	let globals = lua.globals();
	globals.set("userdata", userdata).unwrap();

	let data = lua.load(
		r#"
			return tostring(userdata)
		"#.to_string().as_str(),
	).eval::<String>().unwrap();

	assert_eq!(data, "172382467385196544")
}
