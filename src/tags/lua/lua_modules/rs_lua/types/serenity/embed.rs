use rlua::{Error, FromLua, UserData, Value};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use tagbot_macros::lua_document;

use crate::tags::lua::{
	lua_modules::rs_lua::types::{
		utils::{functions::get_option_from_table, types::ConstructableFromLuaContext},
		Requireable,
	},
	util::dump_table,
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
			if let Some(Value::String(name_str)) = table.get("name")? {
				if let Ok(name_str) = name_str.to_str() {
					author.name(name_str.to_string());
				}
			}

			if let Some(Value::String(url)) = table.get("url")? {
				if let Ok(url) = url.to_str() {
					author.url(url.to_string());
				}
			}

			if let Some(Value::String(icon_url)) = table.get("icon_url")? {
				if let Ok(icon_url) = icon_url.to_str() {
					author.icon_url(icon_url.to_string());
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

impl FromLua<'_> for TBEmbed {
	fn from_lua(value: Value<'_>, lua: &'_ rlua::Lua) -> rlua::Result<Self> {
		let tb_embed = value.as_userdata().unwrap();

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

impl From<FromLuaCreateEmbedAuthor> for CreateEmbedAuthor {
	fn from(val: FromLuaCreateEmbedAuthor) -> Self {
		val.0 // Unwrap the inner CreateEmbedAuthor
	}
}

impl<'lua> ConstructableFromLuaContext<'lua, rlua::Table<'lua>> for TBEmbed {
	fn new(value: rlua::Table<'lua>, ctx: rlua::Context<'lua>) -> Self {
		let mut create_embed = CreateEmbed { ..Default::default() };

		println!(
			"Creating new embed with value {:#?}",
			dump_table(&value.clone())
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
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        // methods.add_method("create", |ctx, this, value: Table| {
        //     this.0.author = value.get("author");
        // });

		methods.add_method("test", |_ctx, this, _: Value| {
			println!("{:#?}", this.0);
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
		let func = ctx.create_function(|ctx2, params: rlua::Table| {
			let value = params;

			println!("Func params are {}", dump_table(&value.clone()));

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
