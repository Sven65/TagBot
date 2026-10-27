use rlua::{IntoLua, MetaMethod, UserData, Value};
use serenity::http::Http;
use serenity::model::guild::Role;
use std::sync::Arc;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{
		convert_constructable, convert_constructable2, convert_type, convert_type_option, lua_todo,
	},
	types::ConstructableFrom2,
};

use super::{colour::TBColour, guild_id::TBGuildId, id::role_id::TBRoleId, role_tags::TBRoleTags};

/// Wrapper for [`serenity::model::guild::Role`]
#[derive(Clone)]
#[lua_document("TBRole", class)]
pub struct TBRole(pub Role, pub Arc<Http>);

impl ConstructableFrom2<Role, Arc<Http>> for TBRole {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `role` - The serenity Role to wrap
	fn new(role: Role, http: Arc<Http>) -> TBRole {
		TBRole(role, http)
	}
}

impl UserData for TBRole {
	#[rustfmt::skip]
	#[lua_document("TBRole", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable::<TBRoleId, _>(this.0.id, ctx)?,
				"guild_id" => convert_constructable2::<TBGuildId, _, Arc<Http>>(this.0.guild_id, this.1.clone(), ctx)?,
				"colour" => convert_constructable::<TBColour, _>(this.0.colour, ctx)?,
				"hoist" => convert_type::<bool>(this.0.hoist, ctx)?,
				"managed" => convert_type::<bool>(this.0.managed, ctx)?,
				"mentionable" => convert_type::<bool>(this.0.mentionable, ctx)?,
				"name" => convert_type::<String>(this.0.name.clone(), ctx)?,
				"permissions" => lua_todo(ctx)?, // TODO(bitflags)
				"position" => convert_type::<u16>(this.0.position, ctx)?,
				"tags" => convert_constructable2::<TBRoleTags, _, Arc<Http>>(this.0.tags.clone(), this.1.clone(), ctx)?,
				"icon" => convert_type_option::<String>(this.0.icon.map(|hash| hash.to_string()), ctx)?,
				"unicode_emoji" => convert_type_option::<String>(this.0.unicode_emoji.clone(), ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
