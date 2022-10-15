use rlua::{MetaMethod, UserData, Value};
use serenity::model::{guild::GuildWelcomeScreen, prelude::GuildWelcomeChannel};
use serenity::prelude::Context as SerenityContext;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::functions::{
	convert_type_option, convert_vec_new,
};
use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::guild_welcome_channel::TBGuildWelcomeChannel;

/// Wrapper for [`serenity::model::guild::GuildWelcomeScreen`]
#[derive(Clone)]
#[lua_document("TBWelcomeScreen", class)]
pub struct TBWelcomeScreen(pub GuildWelcomeScreen, pub SerenityContext);

impl ConstructableFrom2<GuildWelcomeScreen, SerenityContext> for TBWelcomeScreen {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `welcome_screen` - The serenity GuildWelcomeScreen to wrap
	/// * `ctx` - The serenity context to wrap
	fn new(welcome_screen: GuildWelcomeScreen, ctx: SerenityContext) -> TBWelcomeScreen {
		TBWelcomeScreen(welcome_screen, ctx)
	}
}

impl UserData for TBWelcomeScreen {
	#[rustfmt::skip]
	#[lua_document("TBWelcomeScreen", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"description" => convert_type_option::<String>(this.0.description.clone(), ctx)?,
				"welcome_channels" => convert_vec_new::<TBGuildWelcomeChannel, GuildWelcomeChannel, _>(this.0.welcome_channels.clone(), this.1.clone(), ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
