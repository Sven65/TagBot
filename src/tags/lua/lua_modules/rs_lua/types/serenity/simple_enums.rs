use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;
use rlua::{MetaMethod, ToLua, UserData, Value};
/// Wrappers for simple enums, that are really only `tostring(...)`
use serenity::model::prelude::{
	ChannelType, DefaultMessageNotificationLevel, MfaLevel, NsfwLevel, PremiumTier,
	VerificationLevel, VideoQualityMode,
};
use tagbot_macros::LuaEnum;

/// Wrapper for [`serenity::model::prelude::VideoQualityMode`]
#[derive(Clone, LuaEnum)]
pub struct TBVideoQualityMode(pub VideoQualityMode);

/// Wrapper for [`serenity::model::guild::DefaultMessageNotificationLevel`]
#[derive(Clone, LuaEnum)]
pub struct TBDefaultMessageNotificationLevel(pub DefaultMessageNotificationLevel);

/// Wrapper for [`serenity::model::guild::MfaLevel`]
#[derive(Clone, LuaEnum)]
pub struct TBMfaLevel(pub MfaLevel);

/// Wrapper for [`serenity::model::prelude::VerificationLevel`]
#[derive(Clone, LuaEnum)]
pub struct TBVerificationLevel(pub VerificationLevel);

/// Wrapper for [`serenity::model::prelude::PremiumTier`]
#[derive(Clone, LuaEnum)]
pub struct TBPremiumTier(pub PremiumTier);

/// Wrapper for [`serenity::model::prelude::NsfwLevel`]
#[derive(Clone, LuaEnum)]
pub struct TBNsfwLevel(pub NsfwLevel);

/// Wrapper for [`serenity::model::channel::ChannelType`]
#[derive(Clone, LuaEnum)]
pub struct TBChannelType(pub ChannelType);
