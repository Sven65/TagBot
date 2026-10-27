// Wraps a serenity channel as lua

use rlua::{IntoLua, MetaMethod, UserData, Value};
use serenity::{http::Http, model::prelude::Channel};
use std::sync::Arc;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{
		convert_constructable2, convert_constructable2_option, convert_type, convert_type_option,
	},
	types::ConstructableFrom2,
};

use super::{
	channel_category::TBChannelCategory, channel_id::TBChannelId, embed::TBEmbed,
	guild_channel::TBGuildChannel, private_channel::TBPrivateChannel,
};

/// Wrapper for a Serenity Channel
/// does the wrapping
///
/// lmao
#[derive(Clone)]
#[lua_document("TBChannel", class)]
pub struct TBChannel(pub Channel, pub Arc<Http>);

impl ConstructableFrom2<Channel, Arc<Http>> for TBChannel {
	/// Creates a new wrapper
	fn new(value: Channel, value2: Arc<Http>) -> Self {
		TBChannel(value, value2)
	}
}

impl UserData for TBChannel {
	#[rustfmt::skip]
    #[allow(unused_doc_comments)]
	#[lua_document("TBChannel", tostring, index, parse_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| Ok(this.0.to_string().into_lua(ctx)));

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable2::<TBChannelId, _, Arc<Http>>(this.0.id(), this.1.clone(), ctx)?,
				"category" => convert_constructable2_option::<TBChannelCategory, _, Arc<Http>>(this.0.to_owned().category(), Some(this.1.clone()), ctx)?,
				"is_nsfw" => convert_type::<bool>(this.0.to_owned().guild().is_some_and(|channel| channel.nsfw), ctx)?,
				"private" => convert_constructable2_option::<TBPrivateChannel, _, Arc<Http>>(this.0.to_owned().private(), Some(this.1.clone()), ctx)?,
				"guild" => convert_constructable2_option::<TBGuildChannel, _, Arc<Http>>(this.0.to_owned().guild(), Some(this.1.clone()), ctx)?,
				"position" => convert_type_option::<u16>(this.0.position(), ctx)?,
				&_ => Value::Nil,
			})
		});
	}
}
