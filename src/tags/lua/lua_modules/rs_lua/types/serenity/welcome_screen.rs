use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::guild::GuildWelcomeScreen;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_type_option, convert_vec},
	types::ConstructableFrom,
};

/// Wrapper for [`serenity::model::guild::GuildWelcomeScreen`]
#[derive(Clone)]
pub struct TBWelcomeScreen(pub GuildWelcomeScreen);

impl ConstructableFrom<GuildWelcomeScreen> for TBWelcomeScreen {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `welcome_screen` - The serenity GuildWelcomeScreen to wrap
	fn new(welcome_screen: GuildWelcomeScreen) -> TBWelcomeScreen {
		TBWelcomeScreen(welcome_screen)
	}
}

impl UserData for TBWelcomeScreen {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().to_lua(ctx)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"description" => convert_type_option(this.0.description, ctx)?,
				"welcome_channels" => convert_vec(this.0.welcome_channels, ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
