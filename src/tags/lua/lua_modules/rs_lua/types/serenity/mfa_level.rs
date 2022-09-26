use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::{guild::MfaLevel};
use tagbot_macros::LuaEnum;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;
/// Wrapper for [`serenity::model::guild::MfaLevel`]
#[derive(Clone, LuaEnum)]
pub struct TBMfaLevel(pub MfaLevel);
