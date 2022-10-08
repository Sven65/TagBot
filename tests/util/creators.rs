use futures::channel::mpsc::unbounded;
use serenity::{
	client::bridge::gateway::ShardMessenger,
	gateway::InterMessage,
	model::{
		prelude::{
			Channel, ChannelCategory, ChannelId, ChannelType, GuildChannel, GuildId, MessageId,
			PermissionOverwrite, ThreadMember, ThreadMetadata, VideoQualityMode,
		},
		Timestamp,
	},
};

use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::{
	channel::TBChannel, channel_category::TBChannelCategory,
};

use serde::Serialize;
use serenity::prelude::Context as SerenityContext;
use std::sync::Arc;

// CONSTS \\

const DEFAULT_GUILD_ID: u64 = 355959445907570698;
const DEFAULT_CHANNEL_ID: u64 = 172382467385196544;
const DEFAULT_MESSAGE_ID: u64 = 1028233747717423174;

// END CONSTS \\

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

#[derive(Serialize)]
struct SerenityChannel {
	pub id: ChannelId,

	pub bitrate: Option<u64>,
	pub parent_id: Option<ChannelId>,
	pub guild_id: GuildId,

	#[serde(rename = "type")]
	pub kind: ChannelType,
	pub last_message_id: Option<MessageId>,
	pub last_pin_timestamp: Option<Timestamp>,
	pub name: String,
	#[serde(default)]
	pub permission_overwrites: Vec<PermissionOverwrite>,

	#[serde(default)]
	pub position: i64,
	pub topic: Option<String>,
	pub user_limit: Option<u64>,
	#[serde(default)]
	pub nsfw: bool,
	#[serde(default)]
	pub rate_limit_per_user: Option<u64>,
	pub rtc_region: Option<String>,
	pub video_quality_mode: Option<VideoQualityMode>,
	#[serde(default, deserialize_with = "message_count_patch")]
	pub message_count: Option<u8>,
	pub member_count: Option<u8>,

	pub thread_metadata: Option<ThreadMetadata>,
	pub member: Option<ThreadMember>,
	pub default_auto_archive_duration: Option<u64>,
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
		guild_id: GuildId(DEFAULT_GUILD_ID),
		id: ChannelId(DEFAULT_CHANNEL_ID),
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

/// Creates a guild channel not in a category
pub fn create_guild_channel() -> TBChannel {
	let channel = SerenityChannel {
		id: ChannelId(DEFAULT_CHANNEL_ID),
		bitrate: None,
		parent_id: None,
		guild_id: GuildId(DEFAULT_GUILD_ID),
		kind: ChannelType::Text,
		last_message_id: Some(MessageId(DEFAULT_MESSAGE_ID)),
		last_pin_timestamp: None,
		name: "Guild Channel".to_string(),
		permission_overwrites: vec![],
		position: 0,
		topic: Some("Channel Topic".to_string()),
		user_limit: None,
		nsfw: false,
		rate_limit_per_user: None,
		rtc_region: None,
		video_quality_mode: None,
		message_count: None,
		member_count: None,
		thread_metadata: None,
		member: None,
		default_auto_archive_duration: None,
	};

	let serialized = serde_json::to_string(&channel).unwrap();

	let guild_channel: Channel = serde_json::from_str(&serialized).unwrap();

	TBChannel(guild_channel, create_context())
}
