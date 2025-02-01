use rlua::{Context, FromLua, MetaMethod, Table, UserData, Value};
use serenity::{
	builder::{CreateEmbed, CreateEmbedAuthor},
	model::prelude::{Embed, EmbedAuthor},
};
use tagbot_macros::lua_document;

use crate::{
	handle_error,
	tags::lua::{
		lua_modules::rs_lua::types::{
			utils::{
				functions::{convert_type, convert_type_option, get_option_from_table},
				types::{ConstructableFrom, ConstructableFromLuaContext},
			},
			Requireable,
		},
		util::dump_table,
	},
};

// Wrapper for [`serenity::model::prelude::Embed`]
#[derive(Clone)]
#[lua_document("TBEmbed", class)]
pub struct TBEmbed(pub CreateEmbed);

#[derive(Clone, Debug)]
pub struct FromLuaCreateEmbedAuthor(pub CreateEmbedAuthor);

impl<'lua> FromLua<'lua> for FromLuaCreateEmbedAuthor {
	fn from_lua(value: Value<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
		if let Value::Table(table) = value {
			let mut author = CreateEmbedAuthor::default();

			// Extract the 'name' field from the Lua table and insert into the HashMap
			if let Some(name) = table.get("name")? {
				if let Value::String(name_str) = name {
					// Convert rlua::String to String
					if let Ok(name_str) = name_str.to_str() {
						author.name(name_str.to_string());
					}
				}
			}

			// Extract the 'url' field from the Lua table
			if let Some(url) = table.get("url")? {
				if let Value::String(url_str) = url {
					// Convert rlua::String to String
					if let Ok(url_str) = url_str.to_str() {
						author.url(url_str.to_string());
					}
				}
			}

			// Extract the 'icon_url' field from the Lua table
			if let Some(icon_url) = table.get("icon_url")? {
				if let Value::String(icon_url_str) = icon_url {
					// Convert rlua::String to String
					if let Ok(icon_url_str) = icon_url_str.to_str() {
						author.icon_url(icon_url_str.to_string());
					}
				}
			}

			Ok(FromLuaCreateEmbedAuthor(author))
		} else {
			Err(rlua::Error::FromLuaConversionError {
				from: "Lua value",
				to: "CreateEmbedAuthor",
				message: Some("Expected a table".to_string()),
			})
		}
	}
}

impl Into<CreateEmbedAuthor> for FromLuaCreateEmbedAuthor {
	fn into(self) -> CreateEmbedAuthor {
		self.0 // Unwrap the inner CreateEmbedAuthor
	}
}

impl<'lua> ConstructableFromLuaContext<'lua, rlua::Table<'lua>> for TBEmbed {
	fn new(value: rlua::Table<'lua>, ctx: rlua::Context<'lua>) -> Self {
		let mut create_embed = CreateEmbed { ..Default::default() };

		println!(
			"Creating new embed with value {:#?}",
			dump_table(value.clone())
		);

		// Borrow `value` for the correct lifetime to avoid the "dropped" error
		let binding = get_option_from_table::<FromLuaCreateEmbedAuthor>(&value, "author", ctx);

		println!("Binding is {:#?}", binding);

		let author = match &binding {
			Ok(Some(author)) => Some(author),
			Ok(None) => None,
			Err(e) => {
				cat_loggr::log_fatal!("Failed to get author: {}", e);
				None
			}
		};

		if author.is_some() {
			println!("We have author: {:#?}", author);

			create_embed.set_author(author.unwrap().0.clone());
		}

		TBEmbed(create_embed)
	}
}

impl UserData for TBEmbed {
	#[rustfmt::skip]
    #[allow(unused_doc_comments)]
    #[lua_document("TBEmbed", parse_comments, index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(_methods: &mut T) {
        // methods.add_method("create", |ctx, this, value: Table| {
        //     this.0.author = value.get("author");
        // });

        // methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
        //     Ok(match value.as_str() {
        //         "author" => convert_type_option::<String>(this.0.author.clone(), ctx)?
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
		let func = ctx.create_function(|ctx2, params: rlua::Table| {
			let value = params;

			println!("Func params are {}", dump_table(value.clone()));

			Ok(TBEmbed::new(value, ctx2))
		});

		let test_func = ctx.create_function(|_, name: String| {
			println!("Hello, {}", name);

			Ok(())
		});

		value.set("new", func.unwrap()).unwrap();
		value.set("hello", test_func.unwrap()).unwrap();

		let func_table = Value::Table(value.clone());

		println!("Func table: {:#?}", func_table);

		func_table
	}
}
