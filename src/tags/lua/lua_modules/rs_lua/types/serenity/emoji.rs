use rlua::{UserData, MetaMethod, ToLua, Value};
use serenity::model::prelude::Emoji;

use crate::tags::lua::lua_modules::rs_lua::types::utils::functions::{convert_type, convert_constructable, convert_constructable_option};

use super::{emoji_id::TBEmojiId, user::TBUser};

#[derive(Clone)]
pub struct TBEmoji(pub Emoji);

impl UserData for TBEmoji {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

        methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&"animated" => convert_type(this.0.animated, ctx)?,
				&"available" => convert_type(this.0.available, ctx)?,
				&"id" => convert_constructable::<TBEmojiId, _>(this.0.id, ctx)?,
				&"name" => convert_type(this.0.name, ctx)?,
				&"managed" => convert_type(this.0.managed, ctx)?,
				&"require_colons" => convert_type(this.0.require_colons, ctx)?,
				// &"roles" => convert_type(this.0.roles, ctx)?,
				&"user" => convert_constructable_option::<TBUser, _>(this.0.user, ctx)?,
				&_ => Value::Nil,
			})
		})
    }
}

impl From<Emoji> for TBEmoji {
    fn from(emoji: Emoji) -> Self {
        Self(emoji)
    }
}