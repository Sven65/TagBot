use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::prelude::{EmojiId};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TBEmojiId(pub EmojiId);

impl UserData for TBEmojiId {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});
    }
}

impl From<EmojiId> for TBEmojiId {
    fn from(id: EmojiId) -> Self {
        Self(id)
    }
}