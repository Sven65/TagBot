use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::guild::DefaultMessageNotificationLevel;
use tagbot_macros::LuaEnum;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;


/// Wrapper for [`serenity::model::guild::DefaultMessageNotificationLevel`]
#[derive(Clone, LuaEnum)]
pub struct TBDefaultMessageNotificationLevel(pub DefaultMessageNotificationLevel);
