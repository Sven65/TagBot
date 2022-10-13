use rlua::{MetaMethod, UserData, Value};
use serenity::{model::guild::RoleTags, prelude::Context as SerenityContext};

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable2_option, convert_constructable_option, convert_type},
	types::ConstructableFrom2,
};

use super::{id::integration_id::TBIntegrationId, user_id::TBUserId};

/// Wrapper for [`serenity::model::guild::RoleTags`]
#[derive(Clone)]
pub struct TBRoleTags(pub RoleTags, pub SerenityContext);

impl ConstructableFrom2<RoleTags, SerenityContext> for TBRoleTags {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `role_tags` - The serenity RoleTags to wrap
	/// * `s_ctx` - The serenity context to wrap
	fn new(role_tags: RoleTags, s_ctx: SerenityContext) -> TBRoleTags {
		TBRoleTags(role_tags, s_ctx)
	}
}

impl UserData for TBRoleTags {
	#[rustfmt::skip]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"bot_id" => convert_constructable2_option::<TBUserId, _, SerenityContext>(this.0.bot_id, Some(this.1.clone()), ctx)?,
				"integration_id" => convert_constructable_option::<TBIntegrationId, _>(this.0.integration_id, ctx)?,
				"premium_subscriber" => convert_type::<bool>(this.0.premium_subscriber, ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
