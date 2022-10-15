use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::model::prelude::{Member, RoleId};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::functions::{
	convert_constructable_option, convert_type, convert_type_option, convert_vec, lua_todo,
};

use super::{id::role_id::TBRoleId, timestamp::TBTimestamp};

/// Wrapper for Serenity Member
#[derive(Clone, Debug)]
#[lua_document("TBMember", class)]
pub struct TBMember(Member);

impl TBMember {
	pub fn new(member: Member) -> TBMember {
		TBMember(member)
	}
}

// This looks wild, but it's needed for indexing lol
impl UserData for TBMember {
	#[rustfmt::skip]
	#[lua_document("TBMember", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"deaf" => convert_type::<bool>(this.0.deaf, ctx)?,
				"joined_at" => convert_constructable_option::<TBTimestamp, _>(this.0.joined_at, ctx)?,
				"mute" => convert_type::<bool>(this.0.mute, ctx)?,
				"nick" => convert_type_option::<String>(this.0.nick.clone(), ctx)?,
				"roles" => convert_vec::<TBRoleId, RoleId>(this.0.roles.clone(), ctx)?,
				"pending" => convert_type::<bool>(this.0.pending, ctx)?,
				"premium_since" => convert_constructable_option::<TBTimestamp, _>(this.0.premium_since, ctx)?,
				"permissions" => lua_todo(ctx)?, // TODO(bitflags)
				"avatar" => convert_type_option::<String>(this.0.avatar.clone(), ctx)?,
				"communication_disabled_until" => convert_constructable_option::<TBTimestamp, _>(this.0.communication_disabled_until, ctx)?,
				&_ => Value::Nil,
			})
		});
	}
}
