use rlua::{UserData, MetaMethod, ToLua, Value};
use serenity::{model::prelude::GuildChannel, prelude::{Context as SerenityContext}};

use super::{utils::{types::{ConstructableFrom2}, functions::{convert_type_option, convert_type}}};

/// Wrapper for a Serenity Guild Channel
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
				&"kind" => convert_type::<&str>(this.0.kind.name(), ctx)?,

 				&_ => Value::Nil,
			})
		})
	}
}