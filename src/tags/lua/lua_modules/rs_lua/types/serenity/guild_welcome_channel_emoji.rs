use rlua::{MetaMethod, UserData, Value};
use serenity::model::guild::GuildWelcomeChannelEmoji;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable, convert_type},
	types::ConstructableFrom,
};

use super::emoji_id::TBEmojiId;

/// Wrapper for [`serenity::model::guild::GuildWelcomeChannelEmoji`]
#[derive(Clone)]
pub struct TBGuildWelcomeChannelEmoji(pub GuildWelcomeChannelEmoji);

impl ConstructableFrom<GuildWelcomeChannelEmoji> for TBGuildWelcomeChannelEmoji {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `emoji` - The serenity GuildWelcomeChannelEmoji to wrap
	fn new(emoji: GuildWelcomeChannelEmoji) -> TBGuildWelcomeChannelEmoji {
		TBGuildWelcomeChannelEmoji(emoji)
	}
}

impl UserData for TBGuildWelcomeChannelEmoji {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(match &this.0 {
				GuildWelcomeChannelEmoji::Custom { name, id } => convert_type(name.clone(), ctx)?,
				GuildWelcomeChannelEmoji::Unicode(name) => convert_type(name.clone(), ctx)?,
				_ => Value::Nil,
			})
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			let data = match &this.0 {
				GuildWelcomeChannelEmoji::Custom { id, name } => match value.as_str() {
					"id" => convert_constructable::<TBEmojiId, _>(id.clone(), ctx)?,
					"name" => convert_type(name.clone(), ctx)?,
					_ => Value::Nil,
				},
				_ => Value::Nil,
			};

			Ok(data)
		})
	}
}
