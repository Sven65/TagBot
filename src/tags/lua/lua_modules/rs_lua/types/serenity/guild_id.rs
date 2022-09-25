use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::prelude::{GuildId};
use serenity::{prelude::{Context as SerenityContext}};

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;


/// Wrapper for [`serenity::model::prelude::GuildId`]
#[derive(Clone)]
pub struct TBGuildId(pub GuildId, pub SerenityContext);

impl TBGuildId {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `guild_id` - The serenity GuildId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	pub fn new(guild_id: GuildId, s_ctx: SerenityContext) -> TBGuildId {
		TBGuildId(guild_id, s_ctx)
	}
}

impl ConstructableFrom2<GuildId, SerenityContext> for TBGuildId {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `guild_id` - The serenity GuildId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	fn new(guild_id: GuildId, s_ctx: SerenityContext) -> TBGuildId {
		TBGuildId(guild_id, s_ctx)
	}
}

impl UserData for TBGuildId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		// methods.add_method("resolve", |ctx, this, _: Value| {
		// 	let channel_id = this.0.clone();
		// 	let s_ctx = this.1.clone();

		// 	let channel = tokio::task::block_in_place(move || {
		// 		return Handle::current().block_on(async move {
		// 			let channel = get_channel(channel_id, s_ctx).await;

		// 			return channel
		// 		});
		// 	});

		// 	if channel.is_err() {
		// 		return Err(LuaError::external("Failed to get channel."));
		// 	}

		// 	Ok(TBChannel(channel.unwrap(), this.1.clone()).to_lua(ctx)?)
		// });
	}
}