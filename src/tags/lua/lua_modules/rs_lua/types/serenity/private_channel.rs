use rlua::{IntoLua, MetaMethod, UserData, Value};
use serenity::{http::Http, model::channel::PrivateChannel};
use std::sync::Arc;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable, convert_constructable2, convert_constructable_option},
	types::ConstructableFrom2,
};

use super::{
	channel_id::TBChannelId, message_id::TBMessageId, simple_enums::TBChannelType,
	timestamp::TBTimestamp, user::TBUser,
};

/// Wrapper for [`serenity::model::channel::PrivateChannel`]
#[derive(Clone)]
#[lua_document("TBPrivateChannel", class)]
pub struct TBPrivateChannel(pub PrivateChannel, pub Arc<Http>);

impl ConstructableFrom2<PrivateChannel, Arc<Http>> for TBPrivateChannel {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `channel` - The serenity PrivateChannel to wrap
	/// * `http` - The serenity http client to wrap
	fn new(channel: PrivateChannel, http: Arc<Http>) -> TBPrivateChannel {
		TBPrivateChannel(channel, http)
	}
}

impl UserData for TBPrivateChannel {
	#[rustfmt::skip]
	#[lua_document("TBPrivateChannel", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable2::<TBChannelId, _, Arc<Http>>(this.0.id, this.1.clone(), ctx)?,
				"last_message_id" => convert_constructable_option::<TBMessageId, _>(this.0.last_message_id, ctx)?,
				"last_pin_timestamp" => convert_constructable_option::<TBTimestamp, _>(this.0.last_pin_timestamp, ctx)?,
				"kind" => convert_constructable::<TBChannelType, _>(this.0.kind, ctx)?,
				"recipient" => convert_constructable::<TBUser, _>(this.0.clone().recipient, ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
