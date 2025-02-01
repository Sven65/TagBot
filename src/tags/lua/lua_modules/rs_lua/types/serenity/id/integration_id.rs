use rlua::{IntoLua, MetaMethod, UserData, Value};
use serenity::model::prelude::IntegrationId;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

#[derive(Clone, Hash, PartialEq, Eq)]
#[lua_document("TBIntegrationId", class)]
pub struct TBIntegrationId(pub IntegrationId);

impl ConstructableFrom<IntegrationId> for TBIntegrationId {
	fn new(value: IntegrationId) -> Self {
		Self(value)
	}
}

impl UserData for TBIntegrationId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});
	}
}

impl From<IntegrationId> for TBIntegrationId {
	fn from(id: IntegrationId) -> Self {
		Self(id)
	}
}
