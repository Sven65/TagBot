use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::{model::{prelude::VideoQualityMode}};
use tagbot_macros::LuaEnum;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;


/// Wrapper for [`serenity::model::id::MessageId`]
#[derive(Clone, LuaEnum)]
pub struct TBVideoQualityMode(pub VideoQualityMode);