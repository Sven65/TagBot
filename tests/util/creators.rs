use futures::channel::mpsc::unbounded;
use serenity::{
	client::bridge::gateway::ShardMessenger,
	gateway::InterMessage,
	model::prelude::{
		Channel, ChannelCategory, ChannelId, ChannelType, Emoji, EmojiId, GuildId, MessageId,
	},
};

use tagbot::tags::lua::lua_modules::rs_lua::types::serenity::{
	channel::TBChannel, channel_category::TBChannelCategory, emoji::TBEmoji,
};

use serenity::prelude::Context as SerenityContext;
use std::sync::Arc;

use super::creator_types::{Category, SerenityChannel, SerenityEmoji};

// CONSTS \\

const DEFAULT_GUILD_ID: u64 = 355959445907570698;
const DEFAULT_CHANNEL_ID: u64 = 172382467385196544;
const DEFAULT_MESSAGE_ID: u64 = 1028233747717423174;
const DEFAULT_EMOJI_ID: u64 = 1028239715465433118;

// END CONSTS \\

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

pub fn create_emoji() -> TBEmoji {
	let emoji = SerenityEmoji {
		animated: false,
		available: true,
		id: EmojiId(DEFAULT_EMOJI_ID),
		name: "funny_emoji".to_string(),
		managed: false,
		require_colons: true,
		roles: vec![],
		user: None,
	};

	let serialized = serde_json::to_string(&emoji).unwrap();

	let tb_emoji: Emoji = serde_json::from_str(&serialized).unwrap();

	TBEmoji(tb_emoji)
}
