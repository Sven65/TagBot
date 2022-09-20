// Wraps a serenity channel as lua


use futures::FutureExt;
use rlua::{UserData, MetaMethod, Value, ToLua, Error as LuaError};
use serenity::{model::prelude::{Channel, ChannelId}, prelude::{Context as SerenityContext}, Error, http::CacheHttp};
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

async fn get_channel(channel_id: &TBChannelId) -> Result<&str, Error> {

	Ok("cock")
}

impl UserData for TBChannelId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_method("resolve", |ctx, this, _: Value| {
			println!("Resolving channel");

			let handle = Handle::current();
			let channel_id = this.clone();
			let channel_to_get = channel_id.0;
			let s_ctx = &channel_id.1;
			let guard = handle.enter();

			println!("http is {:#?}", &s_ctx.http);

			let channel_result = futures::executor::block_on(async {
				println!("create task {:#?}", channel_id.0);
				let channel_task = channel_to_get.to_channel(&s_ctx.http);
				// let channel_task = get_channel(&channel_id);
				println!("task dreated");


				let channel = channel_task.await;

				println!("task done {:#?}", channel);

				channel
			});
			println!("adter task");


			// if channel_result.is_err() {
			// 	println!("Failed to get channel.");
			// 	return Ok(rlua::Nil);
			// }


			Ok(channel_result.unwrap().to_string().to_lua(ctx)?)
		});

		// methods.add_method("resolve", (move |ctx, this, _: Value| {
		// 		// let s_ctx = this.1;
		// 		// let channel = this.0.to_channel(&s_ctx.http).await;

		// 		// return channel.unwrap();
		// 	});

		// 	Ok(v)
		// })
	}
}

// impl From<TBChannelId> for TBChannel {
//     fn from(channel_id: TBChannelId) -> Self {
//         let channel = channel_id.0.to_channel(cache_http)
//     }
// }