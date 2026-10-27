use rlua::{MetaMethod, UserData, Value};
use serenity::model::guild::PartialThreadMember;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::functions::{
	convert_constructable, lua_todo,
};
use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

use super::timestamp::TBTimestamp;

/// Wrapper for [`serenity::model::guild::PartialThreadMember`]
#[derive(Clone)]
#[lua_document("TBThreadMember", class)]
pub struct TBThreadMember(pub PartialThreadMember);

impl ConstructableFrom<PartialThreadMember> for TBThreadMember {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `thread_member` - The serenity PartialThreadMember to wrap
	fn new(thread_member: PartialThreadMember) -> TBThreadMember {
		TBThreadMember(thread_member)
	}
}

impl UserData for TBThreadMember {
	#[rustfmt::skip]
	#[lua_document("TBThreadMember", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0.to_string().to_lua(ctx)?)
		// });

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			// this.0.flags.contains(ThreadMemberFlags::NOTIFICATIONS)

			Ok(match value.as_str() {
				"join_timestamp" => convert_constructable::<TBTimestamp, _>(this.0.join_timestamp, ctx)?,
				"flags" => lua_todo(ctx)?, // TODO(bitflags)
				_ => Value::Nil,
			})
		})
	}
}
