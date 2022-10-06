use futures::channel::mpsc::unbounded;
use serenity::{
	client::bridge::gateway::ShardMessenger,
	gateway::InterMessage,
	model::prelude::{ChannelCategory, ChannelId, ChannelType, GuildId, PermissionOverwrite},
};

use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::channel_category::TBChannelCategory;

use serde::Serialize;
use serenity::prelude::Context as SerenityContext;
use std::sync::Arc;

#[derive(Serialize)]
struct Category {
	pub guild_id: GuildId,
	pub id: ChannelId,
	#[serde(rename = "type")]
	pub kind: ChannelType,
	pub name: String,
	pub nsfw: bool,
	pub parent_id: Option<ChannelId>,
	pub permission_overwrites: Vec<PermissionOverwrite>,
	pub position: i64,
}

pub fn create_context() -> SerenityContext {
	serenity::prelude::Context {
		cache: Default::default(),
		data: Default::default(),
		http: Arc::new(serenity::http::HttpBuilder::new("123").build()),
		shard: ShardMessenger::new(unbounded::<InterMessage>().0),
		shard_id: Default::default(),
	}
}

pub fn create_channel_category() -> TBChannelCategory {
	let category = Category {
		guild_id: GuildId(355959445907570698),
		id: ChannelId(172382467385196544),
		kind: ChannelType::Category,
		name: "Channel Category".to_string(),
		nsfw: false,
		parent_id: None,
		permission_overwrites: vec![],
		position: 1,
	};

	let serialized = serde_json::to_string(&category).unwrap();

	let channel_category: ChannelCategory = serde_json::from_str(&serialized).unwrap();

	TBChannelCategory(channel_category, create_context())
}
