// Wraps a serenity channel as lua


use rlua::{UserData, MetaMethod, Value, ToLua, Error as LuaError};
use serenity::{model::prelude::{Channel, ChannelId}, prelude::{Context as SerenityContext}, Error};
use tagbot_macro::ud_index;
use tokio::runtime::{Handle};

use crate::tags;

/// Wrapper for serenity ChannelId
#[derive(Clone)]
pub struct TBChannelId(ChannelId, SerenityContext);

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

/// Wrapper for a Serenity Channel
#[derive(Clone, Debug)]
pub struct TBChannel(Channel);


impl TBChannel {
	/// Creates a new wrapper
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

			Ok(TBChannel(channel.unwrap()).to_lua(ctx)?)
			
			// Ok("sds".to_lua(ctx)?)
		});
	}
}


// #[ud_index("third_field", AccessType::Field, "field3", LuaType::StringOrNil)]
#[ud_index("id", AccessType::Function, "id", LuaType::Convert, tags::lua::lua_modules::rs_lua::types::channel:TBChannelId)]
#[ud_index("category", AccessType::Function, "category", LuaType::ConvertOrNil, tags::lua::lua_modules::rs_lua::types::channel_category::TBChannelCategory)]
// #[ud_index("another_field", AccessType::Field, "field_2", LuaType::StringOrNil)]
impl UserData for TBChannel {
	// fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
	// 	methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
	// 		Ok(match &value.as_str() {
	// 			&"c" => {
	// 				let gotten_value = this.0.to_owned().id();
	// 				let cloned_value = gotten_value.clone().unwrap();

	// 				let converted_value = tags::lua::lua_modules::rs_lua::types::channel_category::TBChannelCategory::new(
	// 					cloned_value,
	// 				);
	// 				converted_value.to_lua(ctx)?
	// 			},
	// 			&_ => Value::Nil,
	// 		})
	// 	})
	// }

}
