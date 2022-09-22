// Wraps a serenity channel as lua


use rlua::{UserData, MetaMethod, Value, ToLua, Error as LuaError};
use serenity::{model::prelude::{Channel, ChannelId}, prelude::{Context as SerenityContext}, Error};
use tokio::runtime::{Handle};


#[derive(Clone)]
pub struct TBChannelId(ChannelId, SerenityContext);

impl TBChannelId {
	pub fn new(channel_id: ChannelId, s_ctx: SerenityContext) -> TBChannelId {
		TBChannelId(channel_id, s_ctx)
	}
}

#[derive(Clone, Debug)]
pub struct TBChannel(Channel);

impl TBChannel {
	pub fn new(channel: Channel) -> TBChannel {
		TBChannel(channel)
	}
}

async fn get_channel(channel_id: ChannelId, s_ctx: SerenityContext) -> Result<Channel, Error> {
	let channel = channel_id.to_channel(&s_ctx.http).await.unwrap();

	Ok(channel)
}

impl UserData for TBChannelId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_method("resolve", |ctx, this, _: Value| {
			let handle = Handle::current();
			let channel_id = this.0.clone();
			let s_ctx = this.1.clone();

			let channel = tokio::task::block_in_place(move || {
				return Handle::current().block_on(async move {
					let channel = get_channel(channel_id, s_ctx).await;

					println!("Got channel now {:#?}", channel);

					return channel
				});
			});

			if channel.is_err() {
				return Err(LuaError::external("Failed to get channel."));
			}

			Ok(TBChannel(channel.unwrap()).to_lua(ctx)?)
		});
	}
}

impl UserData for TBChannel {

}

// impl From<TBChannelId> for TBChannel {
//     fn from(channel_id: TBChannelId) -> Self {
//         let channel = channel_id.0.to_channel(cache_http)
//     }
// }