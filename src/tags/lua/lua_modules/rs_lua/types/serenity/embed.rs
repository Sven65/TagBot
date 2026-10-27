use cat_loggr::log_debug;
use rlua::{
	Error, FromLua, UserData,
	Value::{self},
};
use serenity::{
	builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
	model::channel::{EmbedAuthor, EmbedFooter},
};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::Requireable;

use super::{colour::TBColour, timestamp::TBTimestamp};

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

// TODO: Make these two generic
pub fn get_author_from_create_embed(embed: &CreateEmbed) -> Option<EmbedAuthor> {
	let embed_value = serde_json::to_value(embed).ok()?;

	embed_value.get("author").and_then(|author_value| {
		log_debug!("author_value {:#?}", author_value);

		serde_json::from_value::<EmbedAuthor>(author_value.clone()).ok()
	})
}

pub fn get_footer_from_create_embed(embed: &CreateEmbed) -> Option<EmbedFooter> {
	let embed_value = serde_json::to_value(embed).ok()?;

	embed_value
		.get("footer")
		.and_then(|footer_value| serde_json::from_value::<EmbedFooter>(footer_value.clone()).ok())
}

impl UserData for TBEmbed {
	#[rustfmt::skip]
    #[allow(unused_doc_comments)]
    #[lua_document("TBEmbed", parse_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		/// @desc Sets the name of the author
		/// @method
		/// @param {string} name The name to set
		methods.add_method_mut("set_author_name", |_ctx, this: &mut TBEmbed, value: String| {
			let author = get_author_from_create_embed(&this.0)
				.map_or_else(|| CreateEmbedAuthor::new(""), CreateEmbedAuthor::from);

			this.0 = this.0.clone().author(author.name(value));


			Ok(())
		});

		/// @desc Sets the authors icon url
		/// @method
		/// @param {string} url The url to set
		methods.add_method_mut("set_author_icon_url", |_ctx, this, value: String| {
			let author = get_author_from_create_embed(&this.0)
				.map_or_else(|| CreateEmbedAuthor::new(""), CreateEmbedAuthor::from);

			this.0 = this.0.clone().author(author.icon_url(value));

			Ok(())
		});

		/// @desc Sets the author url
		/// @method
		/// @param {string} url The Url name to set
		methods.add_method_mut("set_author_url", |_ctx, this, value: String| {
			let author = get_author_from_create_embed(&this.0)
				.map_or_else(|| CreateEmbedAuthor::new(""), CreateEmbedAuthor::from);

			this.0 = this.0.clone().author(author.url(value));

			Ok(())
		});

		/// @desc Sets the left side colour of the embed
		/// @method
		/// @param {[TBColour](TBColour.md)} colour The colour to set
		methods.add_method_mut("set_colour", |_ctx, this, value: TBColour| {
			this.0 = this.0.clone().colour(value);
			Ok(())
		});

		/// @desc Sets the description of the embed
		/// @method
		/// @param {string} description The description
		methods.add_method_mut("set_description", |_ctx, this, value: Value| {
			this.0 = this.0.clone().description(value.to_string()?);
			Ok(())
		});


		/// @desc Sets the image for the embed
		/// @method
		/// @param {string} url The URL to the image
		methods.add_method_mut("set_image", |_ctx, this, value: String| {
			this.0 = this.0.clone().image(value);
			Ok(())
		});

		/// @desc Sets the thumbnail of the embed
		/// @method
		/// @param {string} url The url of the thumbnail
		methods.add_method_mut("set_thumbnail", |_ctx, this, value: String| {
			this.0 = this.0.clone().thumbnail(value);
			Ok(())
		});

		/// @desc Sets the timestamp of the embed
		/// @method
		/// @param {[TBTimestamp](TBTimestamp.md)} timestamp The timestamp to set
		methods.add_method_mut("set_timestamp", |_ctx, this, value: TBTimestamp| {
			this.0 = this.0.clone().timestamp(value);
			Ok(())
		});

		/// @desc Sets the title of the embed
		/// @method
		/// @param {String} title The title to set
		methods.add_method_mut("set_title", |_ctx, this, value: String| {
			this.0 = this.0.clone().title(value);
			Ok(())
		});

		/// @desc Sets the url of the embed to direct to when clicking the title
		/// @method
		/// @param {String} url The url to set
		methods.add_method_mut("set_url", |_ctx, this, value: String| {
			this.0 = this.0.clone().url(value);
			Ok(())
		});

		/// @desc Sets the embed footers text
		/// @method
		/// @param {string} text The text to set
		methods.add_method_mut("set_footer_text", |_ctx, this: &mut TBEmbed, value: String| {
			let footer = get_footer_from_create_embed(&this.0)
				.map_or_else(|| CreateEmbedFooter::new(""), CreateEmbedFooter::from);

			this.0 = this.0.clone().footer(footer.text(value));

			Ok(())
		});

		/// @desc Set the footer icon URL. This only supports HTTP(S).
		/// @method
		/// @param {string} url The url to set
		methods.add_method_mut("set_footer_icon_url", |_ctx, this, value: String| {
			let footer = get_footer_from_create_embed(&this.0)
				.map_or_else(|| CreateEmbedFooter::new(""), CreateEmbedFooter::from);

			this.0 = this.0.clone().footer(footer.icon_url(value));

			Ok(())
		});

		/// @desc Adds a field to the embed
		/// @method
		/// @param {string} name The title of the field
		/// @param {string} value The value of the field
		/// @param {bool?} inline Optional if the field should be inline
		methods.add_method_mut("add_field", |_ctx, this, (name, value, inline): (String, String, Option<bool>)| {
			this.0 = this.0.clone().field(name, value, inline.unwrap_or(false));

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
		/// @return {TBEmbed} The new embed
		let func = ctx.create_function(|_ctx2, _params: rlua::Table| Ok(TBEmbed(CreateEmbed::default())));

		value.set("new", func.unwrap()).unwrap();

		Value::Table(value.clone())
	}
}
