use rlua::{UserData, Value};
use serenity::model::permissions::Permissions;
// use tagbot_macros::TBBitflag;

// #[derive(TBBitflag)]
pub struct TBPermissions(pub Permissions);

// TODO
impl UserData for TBPermissions {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_method("empty", |ctx, this, _: Value| {})
	}
}
