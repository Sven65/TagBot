// Wraps a serenity channel as lua

use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::prelude::{Channel, ChannelId};


#[derive(Clone, Debug)]
pub struct TBChannelId(ChannelId);

impl TBChannelId {
	pub fn new(channel_id: ChannelId) -> TBChannelId {
		TBChannelId(channel_id)
	}
}

#[derive(Clone, Debug)]
pub struct TBChannel(Channel);

impl TBChannel {
	pub fn new(channel: Channel) -> TBChannel {
		TBChannel(channel)
	}
}

impl UserData for TBChannelId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});
	}
}

// impl From<TBChannelId> for TBChannel {
//     fn from(channel_id: TBChannelId) -> Self {
//         let channel = channel_id.0.to_channel(cache_http)
//     }
// }