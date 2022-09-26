use rlua::{UserData, MetaMethod, ToLua, Value};
use serenity::model::prelude::Emoji;

#[derive(Clone)]
pub struct TBEmoji(pub Emoji);

impl UserData for TBEmoji {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});
    }
}

impl From<Emoji> for TBEmoji {
    fn from(emoji: Emoji) -> Self {
        Self(emoji)
    }
}