use rlua::{
	Error, FromLua, UserData,
	Value::{self, Nil},
};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::Requireable;

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

impl UserData for TBEmbed {
	#[rustfmt::skip]
    #[allow(unused_doc_comments)]
    #[lua_document("TBEmbed", parse_comments, index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        // methods.add_method("create", |ctx, this, value: Table| {
        //     this.0.author = value.get("author");
        // });

		methods.add_method("test", |_ctx, this, ()| {
			println!("Embed is currently {:#?}", this.0);
			Ok(())
		});

		// methods.add_method_mut("set_author", |_ctx, this: &mut TBEmbed, (name, icon_url, url): (String, Option<String>, Option<String>)| {
		// 	let mut author = &mut CreateEmbedAuthor::default();
			
		// 	author.name(value);


			
		// 	Ok(())
		// });

		methods.add_method_mut("set_author_name", |_ctx, this: &mut TBEmbed, value: String| {
			this.0.author(|prev| {
				println!("Prev for author name is {:#?}", prev);
				
				prev.name(value);
				prev
			});
			Ok(())
		});

		methods.add_method_mut("set_author_icon_url", |_ctx, this, value: String| {
			this.0.author(|prev| {
				println!("Prev for icon url is {:#?}", prev);
				prev.icon_url(value);
				prev
			});
			Ok(())
		});

        // methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
        //     Ok(match value.as_str() {
        //         "author_name" => convert_type::<String>(this.0.0, ctx)?,
        //     })
        // });
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

		Value::Table(value)
	}
}
