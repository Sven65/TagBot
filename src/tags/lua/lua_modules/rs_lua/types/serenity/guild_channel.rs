use rlua::{IntoLua, MetaMethod, UserData, Value};
use serenity::{model::prelude::GuildChannel, prelude::Context as SerenityContext};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{
		convert_constructable2, convert_constructable2_option, convert_constructable_option,
		convert_type, convert_type_option,
	},
	types::ConstructableFrom2,
};

use super::{
	channel_id::TBChannelId, guild_id::TBGuildId, message_id::TBMessageId,
	simple_enums::TBVideoQualityMode, thread_member::TBThreadMember,
	thread_metadata::TBThreadMetadata, timestamp::TBTimestamp,
};

/// Wrapper for a [`serenity::model::prelude::GuildChannel`]
#[derive(Clone)]
#[lua_document("TBGuildChannel", class)]
pub struct TBGuildChannel(pub GuildChannel, pub SerenityContext);

impl ConstructableFrom2<GuildChannel, SerenityContext> for TBGuildChannel {
	/// Creates a new wrapper
	fn new(value: GuildChannel, value2: SerenityContext) -> Self {
		TBGuildChannel(value, value2)
	}
}

impl UserData for TBGuildChannel {
	#[rustfmt::skip]
	#[lua_document("TBGuildChannel", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"bitrate" => convert_type_option::<u64>(this.0.bitrate, ctx)?,
				"parent_id" => convert_constructable2_option::<TBChannelId, _, SerenityContext>(this.0.parent_id, Some(this.1.clone()), ctx)?,
				"guild_id" => convert_constructable2::<TBGuildId, _, SerenityContext>(this.0.guild_id, this.1.clone(), ctx)?,
				"kind" => convert_type::<&str>(this.0.kind.name(), ctx)?,
				"last_message_id" => convert_constructable_option::<TBMessageId, _>(this.0.last_message_id, ctx)?,
				"last_pin_timestamp" => convert_constructable_option::<TBTimestamp, _>(this.0.last_pin_timestamp, ctx)?,
				"name" => convert_type::<String>(this.0.name.to_string(), ctx)?,
				"position" => convert_type::<i64>(this.0.position, ctx)?,
				"topic" => convert_type_option::<String>(this.0.topic.clone(), ctx)?,
				"user_limit" => convert_type_option::<u64>(this.0.user_limit, ctx)?,
				"nsfw" => convert_type::<bool>(this.0.nsfw, ctx)?,
				"rate_limit_per_user" => convert_type_option::<u64>(this.0.rate_limit_per_user, ctx)?,
				"rtc_region" => convert_type_option::<String>(this.0.rtc_region.clone(), ctx)?,
				"video_quality_mode" => convert_constructable_option::<TBVideoQualityMode, _>(this.0.video_quality_mode, ctx)?,
				"message_count" => convert_type_option::<u8>(this.0.message_count, ctx)?,
				"member_count" => convert_type_option::<u8>(this.0.member_count, ctx)?,
				"thread_metadata" => convert_constructable_option::<TBThreadMetadata, _>(this.0.thread_metadata, ctx)?,
				"member" => convert_constructable2_option::<TBThreadMember, _, SerenityContext>(this.0.member.clone(), Some(this.1.clone()), ctx)?,
				"default_auto_archive_duration" => convert_type_option::<u64>(this.0.default_auto_archive_duration, ctx)?,

 				&_ => Value::Nil,
			})
		})
	}
}
