use rlua::{MetaMethod, UserData, Value};
use serenity::model::guild::GuildWelcomeChannel;
use serenity::prelude::Context as SerenityContext;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable2, convert_constructable_option, convert_type},
	types::ConstructableFrom2,
};

use super::{channel_id::TBChannelId, guild_welcome_channel_emoji::TBGuildWelcomeChannelEmoji};

/// Wrapper for [`serenity::model::guild::GuildWelcomeChannel`]
#[derive(Clone)]
#[lua_document("TBGuildWelcomeChannel", class)]
pub struct TBGuildWelcomeChannel(pub GuildWelcomeChannel, pub SerenityContext);

impl ConstructableFrom2<GuildWelcomeChannel, SerenityContext> for TBGuildWelcomeChannel {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `welcome_channel` - The serenity GuildWelcomeChannel to wrap
	fn new(
		welcome_channel: GuildWelcomeChannel,
		context: SerenityContext,
	) -> TBGuildWelcomeChannel {
		TBGuildWelcomeChannel(welcome_channel, context)
	}
}

impl UserData for TBGuildWelcomeChannel {
	#[rustfmt::skip]
	#[lua_document("TBGuildWelcomeChannel", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"channel_id" => convert_constructable2::<TBChannelId, _, SerenityContext>(this.0.channel_id, this.1.clone(), ctx)?,
				"description" => convert_type::<&str>(this.0.description.as_str(), ctx)?,
				"emoji" => convert_constructable_option::<TBGuildWelcomeChannelEmoji, _>(this.0.emoji.clone(), ctx)?,
				&_ => Value::Nil,
			})
		})
	}
}
