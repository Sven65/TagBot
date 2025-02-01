use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;
use rlua::{IntoLua, MetaMethod, UserData, Value};
/// Wrappers for simple enums, that are really only `tostring(...)`
use serenity::model::{
	prelude::{
		ChannelType, DefaultMessageNotificationLevel, MfaLevel, NsfwLevel, PremiumTier,
		VerificationLevel, VideoQualityMode,
	},
	sticker::{StickerFormatType, StickerType},
};
use tagbot_macros::{lua_document, LuaEnum};

/// Wrapper for [`serenity::model::prelude::VideoQualityMode`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBVideoQualityMode", class)]
pub struct TBVideoQualityMode(pub VideoQualityMode);

/// Wrapper for [`serenity::model::guild::DefaultMessageNotificationLevel`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBDefaultMessageNotificationLevel", class)]
pub struct TBDefaultMessageNotificationLevel(pub DefaultMessageNotificationLevel);

/// Wrapper for [`serenity::model::guild::MfaLevel`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBMfaLevel", class)]
pub struct TBMfaLevel(pub MfaLevel);

/// Wrapper for [`serenity::model::prelude::VerificationLevel`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBVerificationLevel", class)]
pub struct TBVerificationLevel(pub VerificationLevel);

/// Wrapper for [`serenity::model::prelude::PremiumTier`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBPremiumTier", class)]
pub struct TBPremiumTier(pub PremiumTier);

/// Wrapper for [`serenity::model::prelude::NsfwLevel`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBNsfwLevel", class)]
pub struct TBNsfwLevel(pub NsfwLevel);

/// Wrapper for [`serenity::model::channel::ChannelType`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBChannelType", class)]
pub struct TBChannelType(pub ChannelType);

/// Wrapper for [`serenity::model::sticker::StickerType`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBStickerType", class)]
pub struct TBStickerType(pub StickerType);

/// Wrapper for [`serenity::model::sticker::StickerFormatType`]
#[derive(Clone, LuaEnum)]
#[lua_document("TBStickerFormatType", class)]
pub struct TBStickerFormatType(pub StickerFormatType);
