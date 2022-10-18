use rlua::{MetaMethod, UserData, Value};
use serenity::{model::sticker::Sticker, prelude::Context as SerenityContext};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{
		convert_constructable, convert_constructable2_option, convert_constructable_option,
		convert_type, convert_type_option, convert_vec,
	},
	types::ConstructableFrom2,
};

use super::{
	guild_id::TBGuildId,
	id::ids::{TBStickerId, TBStickerPackId},
	simple_enums::{TBStickerFormatType, TBStickerType},
	user::TBUser,
};

/// Wrapper for [`serenity::model::sticker::Sticker`]
#[derive(Clone)]
#[lua_document("TBSticker", class)]
pub struct TBSticker(pub Sticker, pub SerenityContext);

impl ConstructableFrom2<Sticker, SerenityContext> for TBSticker {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `sticker` - The serenity Sticker to wrap
	/// * `s_ctx` - SerenityContext to use when resolving
	fn new(sticker: Sticker, s_ctx: SerenityContext) -> TBSticker {
		TBSticker(sticker, s_ctx)
	}
}

impl UserData for TBSticker {
	#[rustfmt::skip]
	#[lua_document("TBSticker", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable::<TBStickerId, _>(this.0.id, ctx)?,
				"pack_id" => convert_constructable_option::<TBStickerPackId, _>(this.0.pack_id, ctx)?,
				"name" => convert_type::<String>(this.0.name.clone(), ctx)?,
				"description" => convert_type_option::<String>(this.0.description.clone(), ctx)?,
				"tags" => convert_vec::<String, _>(this.0.tags.clone(), ctx)?,
				"kind" => convert_constructable::<TBStickerType, _>(this.0.kind, ctx)?,
				"format_type" => convert_constructable::<TBStickerFormatType, _>(this.0.format_type, ctx)?,
				"available" => convert_type::<bool>(this.0.available, ctx)?,
				"guild_id" => convert_constructable2_option::<TBGuildId, _, SerenityContext>(this.0.guild_id, Some(this.1.clone()), ctx)?,
				"user" => convert_constructable_option::<TBUser, _>(this.0.user.clone(), ctx)?,
				"sort_value" => convert_type_option::<u64>(this.0.sort_value, ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
