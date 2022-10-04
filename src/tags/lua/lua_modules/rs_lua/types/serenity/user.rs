// Wraps a serenity user as a lua variable

use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::prelude::User;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_type, convert_type_option},
	types::ConstructableFrom,
};

#[derive(Clone, Debug)]
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
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				// &"id" => this.0.id.to_string().to_lua(ctx)?,
				"id" => convert_type::<String>(this.0.id.to_string(), ctx)?,
				"name" => this.0.name.clone().to_lua(ctx)?,
				"avatar" => convert_type_option(this.0.avatar.clone(), ctx)?,
				"banner" => convert_type_option(this.0.banner.clone(), ctx)?,
				"bot" => this.0.bot.to_lua(ctx)?,
				"discriminator" => this.0.discriminator.to_lua(ctx)?,
				"tag" => this.0.tag().to_lua(ctx)?,
				&_ => Value::Nil,
			})
		});
	}
}
