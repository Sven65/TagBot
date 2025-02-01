use rlua::{IntoLua, MetaMethod, UserData, Value};
use serenity::model::id::MessageId;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

/// Wrapper for [`serenity::model::id::MessageId`]
#[derive(Clone)]
#[lua_document("TBMessageId", class)]
pub struct TBMessageId(pub MessageId);

impl ConstructableFrom<MessageId> for TBMessageId {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `message_id` - The serenity MessageId to wrap
	fn new(message_id: MessageId) -> TBMessageId {
		TBMessageId(message_id)
	}
}

impl UserData for TBMessageId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});
	}
}
