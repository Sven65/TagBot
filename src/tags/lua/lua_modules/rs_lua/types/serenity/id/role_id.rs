use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::prelude::RoleId;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

#[derive(Clone, Hash, PartialEq, Eq)]
#[lua_document("TBRoleId", class)]
pub struct TBRoleId(pub RoleId);

impl ConstructableFrom<RoleId> for TBRoleId {
	fn new(value: RoleId) -> Self {
		Self(value)
	}
}

impl UserData for TBRoleId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().to_lua(ctx)
		});
	}
}

impl From<RoleId> for TBRoleId {
	fn from(id: RoleId) -> Self {
		Self(id)
	}
}
