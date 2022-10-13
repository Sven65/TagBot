use rlua::{MetaMethod, UserData, Value};
use serenity::{
	model::{
		guild::PartialGuild,
		prelude::{Emoji, EmojiId, Role, RoleId},
	},
	prelude::Context as SerenityContext,
};

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{
		convert_constructable, convert_constructable2, convert_constructable2_option,
		convert_hashmap_types, convert_hashmap_types_with_new, convert_type, convert_type_option,
		lua_todo,
	},
	types::ConstructableFrom2,
};

use super::{
	channel_id::TBChannelId,
	emoji::TBEmoji,
	emoji_id::TBEmojiId,
	guild_id::TBGuildId,
	id::role_id::TBRoleId,
	role::TBRole,
	simple_enums::{
		TBDefaultMessageNotificationLevel, TBMfaLevel, TBNsfwLevel, TBPremiumTier,
		TBVerificationLevel,
	},
	user_id::TBUserId,
	welcome_screen::TBWelcomeScreen,
};

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
	#[rustfmt::skip]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0. .to_lua(ctx)?)
		// });
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable2::<TBGuildId, _, _>(this.0.id, this.1.clone(), ctx)?,
				"afk_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.afk_channel_id, Some(this.1.clone()), ctx)?,
				"afk_timeout" => convert_type(this.0.afk_timeout, ctx)?,
				"default_message_notifications" => convert_constructable::<TBDefaultMessageNotificationLevel, _>(this.0.default_message_notifications, ctx)?,
				"widget_enabled" => convert_type_option(this.0.widget_enabled, ctx)?,
				"widget_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.widget_channel_id, Some(this.1.clone()), ctx)?,
				"emojis" => convert_hashmap_types::<TBEmojiId, TBEmoji, EmojiId, Emoji>(this.0.emojis.clone(), ctx)?,
				"features" => convert_type(this.0.features.clone(), ctx)?,
				"icon" => convert_type_option::<String>(this.0.icon.clone(), ctx)?,
				"mfa_level" => convert_constructable::<TBMfaLevel, _>(this.0.mfa_level, ctx)?,
				"name" => convert_type(this.0.name.clone(), ctx)?,
				"owner_id" => convert_constructable2::<TBUserId, _, _>(this.0.owner_id, this.1.clone(), ctx)?,
				"roles" => convert_hashmap_types_with_new::<TBRoleId, TBRole, SerenityContext, RoleId, Role>(this.0.roles.clone(), this.1.clone(), ctx)?,
				"splash" => convert_type_option(this.0.splash.clone(), ctx)?,
				"discovery_splash" => convert_type_option(this.0.discovery_splash.clone(), ctx)?,
				"system_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.system_channel_id, Some(this.1.clone()), ctx)?,
				"system_channel_flags" => lua_todo(ctx)?, // TODO
				"rules_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.rules_channel_id, Some(this.1.clone()), ctx)?,
				"public_updates_channel_id" => convert_constructable2_option::<TBChannelId, _, _>(this.0.public_updates_channel_id, Some(this.1.clone()), ctx)?,
				"verification_level" => convert_constructable::<TBVerificationLevel, _>(this.0.verification_level, ctx)?,
				"description" => convert_type_option(this.0.description.clone(), ctx)?,
				"premium_tier" => convert_constructable::<TBPremiumTier, _>(this.0.premium_tier, ctx)?,
				"premium_subscription_count" => convert_type(this.0.premium_subscription_count, ctx)?,
				"banner" => convert_type_option(this.0.banner.clone(), ctx)?,
				"vanity_url_code" => convert_type_option(this.0.vanity_url_code.clone(), ctx)?,
				"welcome_screen" => convert_constructable2_option::<TBWelcomeScreen, _, SerenityContext>(this.0.welcome_screen.clone(), Some(this.1.clone()), ctx)?,
				"approximate_member_count" => convert_type_option(this.0.approximate_member_count, ctx)?,
				"approximate_presence_count" => convert_type_option(this.0.approximate_presence_count, ctx)?,
				"nsfw_level" => convert_constructable::<TBNsfwLevel, _>(this.0.nsfw_level, ctx)?,
				"max_video_channel_users" => convert_type_option(this.0.max_video_channel_users, ctx)?,
				"max_presences" => convert_type_option(this.0.max_presences, ctx)?,
				"max_members" => convert_type_option(this.0.max_members, ctx)?,
				"stickers" => lua_todo(ctx)?, // TODO
				&_ => Value::Nil,
			})
		})

	}
}
