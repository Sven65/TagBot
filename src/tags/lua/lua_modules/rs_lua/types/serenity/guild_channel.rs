use rlua::{UserData, MetaMethod, ToLua, Value};
use serenity::{model::prelude::{GuildChannel}, prelude::{Context as SerenityContext}};

use crate::tags::lua::lua_modules::rs_lua::types::utils::{functions::{convert_type_option, convert_constructable2, convert_constructable2_option, convert_type, convert_constructable_option}, types::ConstructableFrom2};

use super::{channel_id::TBChannelId, guild_id::TBGuildId, message_id::TBMessageId, timestamp::TBTimestamp};

/// Wrapper for a [`serenity::model::prelude::GuildChannel`]
#[derive(Clone)]
pub struct TBGuildChannel(pub GuildChannel, pub SerenityContext);

impl ConstructableFrom2<GuildChannel, SerenityContext> for TBGuildChannel {
	/// Creates a new wrapper
   	fn new(value: GuildChannel, value2: SerenityContext) -> Self {
        TBGuildChannel(value, value2)
    }
}

impl UserData for TBGuildChannel {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&"bitrate" => convert_type_option::<u64>(this.0.bitrate, ctx)?,
				&"parent_id" => convert_constructable2_option::<TBChannelId, _, SerenityContext>(this.0.parent_id, Some(this.1.clone()), ctx)?,
				&"guild_id" => convert_constructable2::<TBGuildId, _, SerenityContext>(this.0.guild_id, this.1.clone(), ctx)?,
				&"kind" => convert_type::<&str>(this.0.kind.name(), ctx)?,
				&"last_message_id" => convert_constructable_option::<TBMessageId, _>(this.0.last_message_id, ctx)?,
				&"last_pin_timestamp" => convert_constructable_option::<TBTimestamp, _>(this.0.last_pin_timestamp, ctx)?,
				&"name" => convert_type(this.0.name.to_string(), ctx)?,
				&"position" => convert_type::<i64>(this.0.position, ctx)?,
				&"topic" => convert_type_option(this.0.topic.clone(), ctx)?,
				&"user_limit" => convert_type_option(this.0.user_limit, ctx)?,
				&"nsfw" => convert_type(this.0.nsfw, ctx)?,
				&"rate_limit_per_user" => convert_type_option(this.0.rate_limit_per_user, ctx)?,
				&"rtc_region" => convert_type_option(this.0.rtc_region.clone(), ctx)?,
				&"video_quality_mode" => Value::Nil,
				&"message_count" => convert_type_option(this.0.message_count, ctx)?,
				&"member_count" => convert_type_option(this.0.member_count, ctx)?,
				&"thread_metadata" => Value::Nil,
				&"member" => Value::Nil,
				&"default_auto_archive_duration" => convert_type_option(this.0.default_auto_archive_duration, ctx)?,

 				&_ => Value::Nil,
			})
		})
	}
}