use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::prelude::Emoji;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::functions::{
	convert_constructable, convert_constructable_option, convert_type, convert_vec,
};

use super::{emoji_id::TBEmojiId, id::role_id::TBRoleId, user::TBUser};

#[derive(Clone)]
#[lua_document("TBEmoji", class)]
pub struct TBEmoji(pub Emoji);

impl UserData for TBEmoji {
	#[rustfmt::skip]
	#[lua_document("TBEmoji", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().to_lua(ctx)
		});

        methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"animated" => convert_type::<bool>(this.0.animated, ctx)?,
				"available" => convert_type::<bool>(this.0.available, ctx)?,
				"id" => convert_constructable::<TBEmojiId, _>(this.0.id, ctx)?,
				"name" => convert_type::<&str>(this.0.name.as_str(), ctx)?,
				"managed" => convert_type::<bool>(this.0.managed, ctx)?,
				"require_colons" => convert_type::<bool>(this.0.require_colons, ctx)?,
                "roles" => convert_vec::<TBRoleId, _>(this.0.roles.clone(), ctx)?,
				"user" => convert_constructable_option::<TBUser, _>(this.0.user.clone(), ctx)?,
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
