use rlua::{Error as LuaError, IntoLua, MetaMethod, UserData, Value};
use serenity::model::prelude::{Channel, ChannelId};
use serenity::{prelude::Context as SerenityContext, Error};
use tagbot_macros::lua_document;
use tokio::runtime::Handle;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::channel::TBChannel;

async fn get_channel(channel_id: ChannelId, s_ctx: SerenityContext) -> Result<Channel, Error> {
	let channel = channel_id.to_channel(&s_ctx.http).await.unwrap();

	Ok(channel)
}

/// Wrapper for serenity ChannelId
#[derive(Clone)]
#[lua_document("TBChannelId", class)]
pub struct TBChannelId(pub ChannelId, pub SerenityContext);

impl TBChannelId {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `channel_id` - The serenity ChannelId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	pub fn new(channel_id: ChannelId, s_ctx: SerenityContext) -> TBChannelId {
		TBChannelId(channel_id, s_ctx)
	}
}

impl ConstructableFrom2<ChannelId, SerenityContext> for TBChannelId {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `channel_id` - The serenity ChannelId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	fn new(channel_id: ChannelId, s_ctx: SerenityContext) -> TBChannelId {
		TBChannelId(channel_id, s_ctx)
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
			let s_ctx = this.1.clone();

			let channel = tokio::task::block_in_place(move || {
				Handle::current().block_on(async move { get_channel(channel_id, s_ctx).await })
			});

			if channel.is_err() {
				return Err(LuaError::external("Failed to get channel."));
			}

			TBChannel(channel.unwrap(), this.1.clone()).into_lua(ctx)
		});
	}
}
