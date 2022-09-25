// Wraps a serenity channel as lua


use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::{model::prelude::{Channel}, prelude::{Context as SerenityContext}};

use crate::tags::lua::lua_modules::rs_lua::types::utils::{types::ConstructableFrom2, functions::{convert_constructable_option, convert_constructable2_option, convert_type_option}};

use super::{channel_category::TBChannelCategory, guild_channel::TBGuildChannel};

/// Wrapper for a Serenity Channel
#[derive(Clone)]
pub struct TBChannel(pub Channel, pub SerenityContext);

impl ConstructableFrom2<Channel, SerenityContext> for TBChannel {
	/// Creates a new wrapper
   	fn new(value: Channel, value2: SerenityContext) -> Self {
        TBChannel(value, value2)
    }
}


impl UserData for TBChannel {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&"category" => convert_constructable_option::<TBChannelCategory, _>(this.0.to_owned().category(), ctx)?,
				&"is_nsfw" => this.0.to_owned().is_nsfw().to_lua(ctx)?,
				// &"private" => this.0.private(),
				&"guild" => convert_constructable2_option::<TBGuildChannel, _, SerenityContext>(this.0.to_owned().guild(), Some(this.1.clone()), ctx)?,
				&"position" => convert_type_option::<i64>(this.0.position(), ctx)?,
 				&_ => Value::Nil,
			})
		})
	}
}
