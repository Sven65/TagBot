use rlua::{UserData, MetaMethod, Error as LuaError, Value, ToLua};
use serenity::model::prelude::{ChannelId, Channel};
use serenity::{prelude::{Context as SerenityContext}, Error};
use tokio::runtime::Handle;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::channel::TBChannel;


async fn get_channel(channel_id: ChannelId, s_ctx: SerenityContext) -> Result<Channel, Error> {
	let channel = channel_id.to_channel(&s_ctx.http).await.unwrap();

	Ok(channel)
}

/// Wrapper for serenity ChannelId
#[derive(Clone)]
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
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_method("resolve", |ctx, this, _: Value| {
			let channel_id = this.0.clone();
			let s_ctx = this.1.clone();

			let channel = tokio::task::block_in_place(move || {
				return Handle::current().block_on(async move {
					let channel = get_channel(channel_id, s_ctx).await;

					return channel
				});
			});

			if channel.is_err() {
				return Err(LuaError::external("Failed to get channel."));
			}

			Ok(TBChannel(channel.unwrap(), this.1.clone()).to_lua(ctx)?)
		});
	}
}