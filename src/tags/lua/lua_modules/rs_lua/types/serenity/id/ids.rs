use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;
use rlua::{MetaMethod, UserData, Value};
use serenity::model::prelude::{StickerId, StickerPackId};
use tagbot_macros::WrappedId;

/// Wrapper for [`serenity::model::id::StickerId`]
#[derive(Clone, Hash, PartialEq, Eq, WrappedId)]
pub struct TBStickerId(pub StickerId);

/// Wrapper for [`serenity::model::id::StickerPackId`]
#[derive(Clone, Hash, PartialEq, Eq, WrappedId)]
pub struct TBStickerPackId(pub StickerPackId);
