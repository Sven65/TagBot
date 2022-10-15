// Wraps a serenity user as a lua variable

use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::prelude::User;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_type, convert_type_option},
	types::ConstructableFrom,
};

#[derive(Clone, Debug)]
#[lua_document("TBUser", class)]
pub struct TBUser(pub User);

impl TBUser {
	pub fn new(user: User) -> TBUser {
		TBUser(user)
	}
}

impl ConstructableFrom<User> for TBUser {
	fn new(value: User) -> Self {
		Self(value)
	}
}

// This looks wild, but it's needed for indexing lol
impl UserData for TBUser {
	#[rustfmt::skip]
	#[lua_document("TBUser", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_type::<String>(this.0.id.to_string(), ctx)?,
				"name" => convert_type::<String>(this.0.name.clone(), ctx)?,
				"avatar" => convert_type_option::<String>(this.0.avatar.clone(), ctx)?,
				"banner" => convert_type_option::<String>(this.0.banner.clone(), ctx)?,
				"bot" => convert_type::<bool>(this.0.bot, ctx)?,
				"discriminator" => convert_type::<u16>(this.0.discriminator, ctx)?,
				"tag" => convert_type::<String>(this.0.tag(), ctx)?,
				&_ => Value::Nil,
			})
		});
	}
}
