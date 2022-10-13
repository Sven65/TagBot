use reql::types::User;
use serde::Serialize;
use serenity::model::{
	prelude::{
		ChannelId, ChannelType, EmojiId, GuildId, MessageId, PermissionOverwrite, RoleId,
		ThreadMember, ThreadMetadata, VideoQualityMode,
	},
	Timestamp,
};

#[derive(Serialize)]
pub struct Category {
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
pub struct SerenityChannel {
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

#[derive(Serialize)]
pub struct SerenityEmoji {
	#[serde(default)]
	pub animated: bool,
	#[serde(default = "default_true")]
	pub available: bool,
	pub id: EmojiId,
	pub name: String,
	#[serde(default)]
	pub managed: bool,
	#[serde(default)]
	pub require_colons: bool,
	#[serde(default)]
	pub roles: Vec<RoleId>,
	pub user: Option<User>,
}
