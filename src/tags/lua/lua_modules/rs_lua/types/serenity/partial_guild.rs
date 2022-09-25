use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::{model::guild::PartialGuild, prelude::{Context as SerenityContext}};

use crate::tags::lua::lua_modules::rs_lua::types::utils::{types::ConstructableFrom2, functions::{convert_constructable, convert_constructable2, convert_constructable2_option, convert_type, convert_type_option}};

use super::{guild_id::TBGuildId, channel_id::TBChannelId, default_message_notification_level::TBDefaultMessageNotificationLevel, mfa_level::TBMfaLevel};

/// Wrapper for [`serenity::model::guild::PartialGuild`]
#[derive(Clone)]
pub struct TBPartialGuild(pub PartialGuild, pub SerenityContext);

impl ConstructableFrom2<PartialGuild, SerenityContext> for TBPartialGuild {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `partial_guild` - The serenity PartialGuild to wrap
	/// * `context` - The serenity context to wrap
	fn new(partial_guild: PartialGuild, context: SerenityContext) -> TBPartialGuild {
		TBPartialGuild(partial_guild, context)
	}
}

impl UserData for TBPartialGuild {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0. .to_lua(ctx)?)
		// });

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&"id" => convert_constructable2::<TBGuildId, _, _>(this.0.id, this.1.clone(), ctx)?,
				&"afk_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.afk_channel_id, Some(this.1.clone()), ctx)?,
				&"afk_timeout" => convert_type(this.0.afk_timeout, ctx)?,
				&"default_message_notifications" => convert_constructable::<TBDefaultMessageNotificationLevel, _>(this.0.default_message_notifications, ctx)?,
				&"widget_enabled" => convert_type_option(this.0.widget_enabled, ctx)?,
				&"widget_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.widget_channel_id, Some(this.1.clone()), ctx)?,
				&"emojis" => todo!(),
				&"features" => todo!(),
				&"icon" => convert_type_option::<String>(this.0.icon.clone(), ctx)?,
				&"mfa_level" => convert_constructable::<TBMfaLevel, _>(this.0.mfa_level, ctx)?,
				&"name" => convert_type(this.0.name.clone(), ctx)?,
				&"owner_id" => todo!(),
				&"roles" => todo!(),
				&"splash" => convert_type_option(this.0.splash.clone(), ctx)?,
				&"discovery_splash" => convert_type_option(this.0.discovery_splash.clone(), ctx)?,
				&"system_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.system_channel_id, Some(this.1.clone()), ctx)?,
				&"system_channel_flags" => todo!(),
				&"rules_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.rules_channel_id, Some(this.1.clone()), ctx)?,
				&"public_updates_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.public_updates_channel_id, Some(this.1.clone()), ctx)?,
				&"verification_level" => todo!(),
				&"description" => convert_type_option(this.0.description.clone(), ctx)?,
				&"premium_tier" => todo!(),
				&"premium_subscription_count" => convert_type(this.0.premium_subscription_count, ctx)?,
				&"banner" => convert_type_option(this.0.banner.clone(), ctx)?,
				&"vanity_url_code" => convert_type_option(this.0.vanity_url_code.clone(), ctx)?,
				&"welcome_screen" => todo!(),
				&"approximate_member_count" => convert_type_option(this.0.approximate_member_count, ctx)?,
				&"approximate_presence_count" => convert_type_option(this.0.approximate_presence_count, ctx)?,
				&"nsfw_level" => todo!(),
				&"max_video_channel_users" => convert_type_option(this.0.max_video_channel_users, ctx)?,
				&"max_presences" => convert_type_option(this.0.max_presences, ctx)?,
				&"max_members" => convert_type_option(this.0.max_members, ctx)?,
				&"stickers" => todo!(),
				&_ => Value::Nil,
			})
		})

	}
}