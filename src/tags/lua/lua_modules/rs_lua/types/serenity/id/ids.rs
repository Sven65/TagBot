use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;
use rlua::{MetaMethod, UserData, Value};
use serenity::model::prelude::{StickerId, StickerPackId};
use tagbot_macros::{lua_document, WrappedId};

/// Wrapper for [`serenity::model::id::StickerId`]
#[derive(Clone, Hash, PartialEq, Eq, WrappedId)]
#[lua_document("TBStickerId", class)]
pub struct TBStickerId(pub StickerId);

/// Wrapper for [`serenity::model::id::StickerPackId`]
#[derive(Clone, Hash, PartialEq, Eq, WrappedId)]
#[lua_document("TBStickerPackId", class)]
pub struct TBStickerPackId(pub StickerPackId);
