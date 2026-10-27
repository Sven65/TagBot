# TBGuildChannel

 Wrapper for a [`serenity::model::prelude::GuildChannel`]
# Attributes
- bitrate :: u32?
- parent_id :: TBChannelId?
- guild_id :: TBGuildId
- kind :: &str
- last_message_id :: TBMessageId?
- last_pin_timestamp :: TBTimestamp?
- name :: String
- position :: u16
- topic :: String?
- user_limit :: u32?
- nsfw :: bool
- rate_limit_per_user :: u16?
- rtc_region :: String?
- video_quality_mode :: TBVideoQualityMode?
- message_count :: u32?
- member_count :: u8?
- thread_metadata :: TBThreadMetadata?
- member :: TBThreadMember?
- default_auto_archive_duration :: u16?
