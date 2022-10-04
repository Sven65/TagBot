use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::prelude::EmojiId;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TBEmojiId(pub EmojiId);

impl ConstructableFrom<EmojiId> for TBEmojiId {
	fn new(value: EmojiId) -> Self {
		Self(value)
	}
}

impl UserData for TBEmojiId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().to_lua(ctx)
		});
	}
}

impl From<EmojiId> for TBEmojiId {
	fn from(id: EmojiId) -> Self {
		Self(id)
	}
}
