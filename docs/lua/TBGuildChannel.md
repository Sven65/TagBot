# TBGuildChannel

 Wrapper for a [`serenity::model::prelude::GuildChannel`]
# Attributes
- bitrate :: u64?
- parent_id :: TBChannelId?
- guild_id :: TBGuildId
- kind :: &str
- last_message_id :: TBMessageId?
- last_pin_timestamp :: TBTimestamp?
- name :: String
- position :: i64
- topic :: String?
- user_limit :: u64?
- nsfw :: bool
- rate_limit_per_user :: u64?
- rtc_region :: String?
- video_quality_mode :: TBVideoQualityMode?
- message_count :: u8?
- member_count :: u8?
- thread_metadata :: TBThreadMetadata?
- member :: TBThreadMember?
- default_auto_archive_duration :: u64?
