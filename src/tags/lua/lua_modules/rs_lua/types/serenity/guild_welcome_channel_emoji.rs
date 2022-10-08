use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::guild::GuildWelcomeChannelEmoji;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

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

/// TODO
impl UserData for TBGuildWelcomeChannelEmoji {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	this.0.to_string().to_lua(ctx)
		// });

		// methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
		// Ok(match value.as_str() {

		// })
		//})
	}
}
