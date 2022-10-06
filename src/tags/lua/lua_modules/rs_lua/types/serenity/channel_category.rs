use rlua::{MetaMethod, UserData, Value};
use serenity::model::prelude::ChannelCategory;
use serenity::prelude::Context as SerenityContext;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable2, convert_constructable2_option, convert_type},
	types::ConstructableFrom2,
};

use super::{channel_id::TBChannelId, guild_id::TBGuildId};

/// Wrapper for a Serenity Channel
#[derive(Clone)]
pub struct TBChannelCategory(pub ChannelCategory, pub SerenityContext);

impl ConstructableFrom2<ChannelCategory, SerenityContext> for TBChannelCategory {
	fn new(value: ChannelCategory, s_ctx: SerenityContext) -> Self {
		TBChannelCategory(value, s_ctx)
	}
}

impl UserData for TBChannelCategory {
	#[rustfmt::skip]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable2::<TBChannelId, _, SerenityContext>(this.0.id, this.1.clone(), ctx)?,
				"guild_id" => convert_constructable2::<TBGuildId, _, SerenityContext>(this.0.guild_id, this.1.clone(), ctx)?,
				"parent_id" => convert_constructable2_option::<TBChannelId, _, SerenityContext>(this.0.parent_id, Some(this.1.clone()), ctx)?,
				"position" => convert_type::<i64>(this.0.position, ctx)?,
				"kind" => convert_type(this.0.kind.name(), ctx)?,
				"name" => convert_type(this.0.name.as_str(), ctx)?,
				"nsfw" => convert_type::<bool>(this.0.nsfw, ctx)?,
				&_ => Value::Nil,
			})
		})
	}
}
