use rlua::{MetaMethod, UserData, Value};
use serenity::model::guild::ThreadMember;
use serenity::prelude::Context as SerenityContext;

use crate::tags::lua::lua_modules::rs_lua::types::utils::functions::{
	convert_constructable, convert_constructable2_option,
};
use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::channel_id::TBChannelId;
use super::timestamp::TBTimestamp;
use super::user_id::TBUserId;

/// Wrapper for [`serenity::model::guild::ThreadMember`]
#[derive(Clone)]
pub struct TBThreadMember(pub ThreadMember, pub SerenityContext);

impl ConstructableFrom2<ThreadMember, SerenityContext> for TBThreadMember {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `thread_member` - The serenity ThreadMember to wrap
	fn new(thread_member: ThreadMember, ctx: SerenityContext) -> TBThreadMember {
		TBThreadMember(thread_member, ctx)
	}
}

impl UserData for TBThreadMember {
	#[rustfmt::skip]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0.to_string().to_lua(ctx)?)
		// });

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			// this.0.flags.contains(ThreadMemberFlags::NOTIFICATIONS)

			Ok(match value.as_str() {
				"id" => convert_constructable2_option::<TBChannelId , _, SerenityContext>(this.0.id, Some(this.1.clone()), ctx)?,
				"user_id" => convert_constructable2_option::<TBUserId, _, SerenityContext>(this.0.user_id, Some(this.1.clone()), ctx)?,
				"join_timestamp" => convert_constructable::<TBTimestamp, _>(this.0.join_timestamp, ctx)?,
				"flags" => todo!(),
				_ => Value::Nil,
			})
		})
	}
}
