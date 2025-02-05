use std::collections::HashMap;

use rlua::{
	Error, FromLua, UserData,
	Value::{self},
};
use serde_json::Value as JsonValue;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::Requireable;

use super::colour::TBColour;

// Wrapper for [`serenity::model::prelude::Embed`]
#[derive(Clone, Debug)]
#[lua_document("TBEmbed", class)]
pub struct TBEmbed(pub CreateEmbed);

impl FromLua<'_> for TBEmbed {
	fn from_lua(value: Value<'_>, _lua: &'_ rlua::Lua) -> rlua::Result<Self> {
		let tb_embed = value.as_userdata();

		if tb_embed.is_none() {
			panic!("Passed value is none");
		}

		let tb_embed = tb_embed.unwrap();

		if !tb_embed.is::<TBEmbed>() {
			return Err(Error::external("Passed type is not TBEmbed"));
		}

		let tb_embed = match tb_embed.take::<TBEmbed>() {
			Ok(embed) => embed,
			Err(_) => return Err(Error::external("Failed to take internal TBEmbed")),
		};

		Ok(tb_embed)
	}
}

pub fn get_author_from_create_embed(embed: &CreateEmbed) -> Option<CreateEmbedAuthor> {
	embed.0.get("author").and_then(|author_value| {
		if let JsonValue::Object(map) = author_value {
			let converted_map: HashMap<&'static str, JsonValue> = map
				.iter()
				.map(|(k, v)| {
					let key: &'static str = Box::leak(k.clone().into_boxed_str());
					(key, v.clone())
				})
				.collect();

			Some(CreateEmbedAuthor(converted_map))
		} else {
			None
		}
	})
}

impl UserData for TBEmbed {
	#[rustfmt::skip]
    #[allow(unused_doc_comments)]
    #[lua_document("TBEmbed", parse_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {

		methods.add_method("test", |_ctx, this, ()| {
			println!("Embed is currently {:#?}", this.0);
			Ok(())
		});

		/// @desc Sets the name of the author
		/// @method
		/// @param {string} name The name to set
		methods.add_method_mut("set_author_name", |_ctx, this: &mut TBEmbed, value: String| {
			let author = &mut get_author_from_create_embed(&this.0).unwrap_or_default();

			author.name(value);

			this.0.set_author(author.to_owned());

			Ok(())
		});

		/// @desc Sets the authors icon url
		/// @method
		/// @param {string} url The url to set
		methods.add_method_mut("set_author_icon_url", |_ctx, this, value: String| {
			let author = &mut get_author_from_create_embed(&this.0).unwrap_or_default();
			
			author.icon_url(value);

			this.0.set_author(author.to_owned());

			Ok(())
		});
    }
}

#[lua_document("TBEmbed", requireable = "embed")]
#[allow(unused_doc_comments)]
impl Requireable for TBEmbed {
	/// @desc Creates a requireable module
	/// @return {module} The embed module
	fn create_module(ctx: rlua::Context) -> rlua::Value {
		let value = ctx.create_table();

		if value.is_err() {
			return rlua::Nil;
		}

		let value = value.unwrap();

		/// @desc Creates a new embed
		/// @function
		/// @param {Embed} params The embed values to create with
		/// @return {TBEmbed} The new embed
		let func = ctx.create_function(|_ctx2, _params: rlua::Table| Ok(TBEmbed(CreateEmbed::default())));

		value.set("new", func.unwrap()).unwrap();

		Value::Table(value.clone())
	}
}
