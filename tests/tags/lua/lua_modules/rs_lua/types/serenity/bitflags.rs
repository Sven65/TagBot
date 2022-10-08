use serenity::model::permissions::Permissions;
use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::bitflags::TBBitflags;

#[test]
fn bitflag_test() {
	let flags = TBBitflags(Permissions);

	// flags.0
}
