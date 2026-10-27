use rlua::{MetaMethod, UserData, Value};
use serenity::http::Http;
use serenity::model::prelude::GuildChannel;
use std::sync::Arc;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable2, convert_constructable2_option, convert_type},
	types::ConstructableFrom2,
};

use super::{channel_id::TBChannelId, guild_id::TBGuildId};

/// Wrapper for a Serenity Channel
#[derive(Clone)]
#[lua_document("TBChannelCategory", class)]
pub struct TBChannelCategory(pub GuildChannel, pub Arc<Http>);

impl ConstructableFrom2<GuildChannel, Arc<Http>> for TBChannelCategory {
	fn new(value: GuildChannel, http: Arc<Http>) -> Self {
		TBChannelCategory(value, http)
	}
}

impl UserData for TBChannelCategory {
	#[rustfmt::skip]
	#[lua_document("TBChannelCategory", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable2::<TBChannelId, _, Arc<Http>>(this.0.id, this.1.clone(), ctx)?,
				"guild_id" => convert_constructable2::<TBGuildId, _, Arc<Http>>(this.0.guild_id, this.1.clone(), ctx)?,
				"parent_id" => convert_constructable2_option::<TBChannelId, _, Arc<Http>>(this.0.parent_id, Some(this.1.clone()), ctx)?,
				"position" => convert_type::<u16>(this.0.position, ctx)?,
				"kind" => convert_type::<&str>(this.0.kind.name(), ctx)?,
				"name" => convert_type::<&str>(this.0.name.as_str(), ctx)?,
				"nsfw" => convert_type::<bool>(this.0.nsfw, ctx)?,
				&_ => Value::Nil,
			})
		})
	}
}
