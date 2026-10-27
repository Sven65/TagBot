use rlua::{Error as LuaError, IntoLua, MetaMethod, UserData, Value};
use serenity::model::prelude::{Channel, ChannelId};
use serenity::{http::Http, Error};
use std::sync::Arc;
use tagbot_macros::lua_document;
use tokio::runtime::Handle;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::channel::TBChannel;

async fn get_channel(channel_id: ChannelId, http: Arc<Http>) -> Result<Channel, Error> {
	let channel = channel_id.to_channel(&http).await.unwrap();

	Ok(channel)
}

/// Wrapper for serenity ChannelId
#[derive(Clone)]
#[lua_document("TBChannelId", class)]
pub struct TBChannelId(pub ChannelId, pub Arc<Http>);

impl TBChannelId {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `channel_id` - The serenity ChannelId to wrap
	/// * `http` - Http client to use when resolving channel
	pub fn new(channel_id: ChannelId, http: Arc<Http>) -> TBChannelId {
		TBChannelId(channel_id, http)
	}
}

impl ConstructableFrom2<ChannelId, Arc<Http>> for TBChannelId {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `channel_id` - The serenity ChannelId to wrap
	/// * `http` - Http client to use when resolving channel
	fn new(channel_id: ChannelId, http: Arc<Http>) -> TBChannelId {
		TBChannelId(channel_id, http)
	}
}

impl UserData for TBChannelId {
	#[lua_document("TBChannelId", parse_comments)]
	#[allow(unused_doc_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});

		/// @desc Resolves the ID to a channel.
		/// @method
		/// @return {TBChannel} A discord channel
		methods.add_method("resolve", |ctx, this, _: Value| {
			let channel_id = this.0;
			let http = this.1.clone();

			let channel = tokio::task::block_in_place(move || {
				Handle::current().block_on(async move { get_channel(channel_id, http).await })
			});

			if channel.is_err() {
				return Err(LuaError::external("Failed to get channel."));
			}

			TBChannel(channel.unwrap(), this.1.clone()).into_lua(ctx)
		});
	}
}
