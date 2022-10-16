# TBPartialGuild

 Wrapper for [`serenity::model::guild::PartialGuild`]
# Attributes
- id :: TBGuildId
- afk_channel_id :: TBChannelId?
- afk_timeout :: u64
- default_message_notifications :: TBDefaultMessageNotificationLevel
- widget_enabled :: bool?
- widget_channel_id :: TBChannelId?
- emojis :: HashMap<TBEmojiId, TBEmoji>
- features :: array[String]
- icon :: String?
- mfa_level :: TBMfaLevel
- name :: String
- owner_id :: TBUserId
- roles :: HashMap<TBRoleId, TBRole>
- splash :: String?
- discovery_splash :: String?
- system_channel_id :: TBChannelId?
- system_channel_flags :: <!> Unknown (Not implemented) <!>
- rules_channel_id :: TBChannelId?
- public_updates_channel_id :: TBChannelId?
- verification_level :: TBVerificationLevel
- description :: String?
- premium_tier :: TBPremiumTier
- premium_subscription_count :: u64
- banner :: String?
- vanity_url_code :: String?
- welcome_screen :: TBWelcomeScreen?
- approximate_member_count :: u64?
- approximate_presence_count :: u64?
- nsfw_level :: TBNsfwLevel
- max_video_channel_users :: u64?
- max_presences :: u64?
- max_members :: u64?
- stickers :: HashMap<TBStickerId, TBSticker>
