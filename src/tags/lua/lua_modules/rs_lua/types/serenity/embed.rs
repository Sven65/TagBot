use rlua::{MetaMethod, Table, UserData};
use serenity::model::prelude::Embed;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_type, convert_type_option},
	types::ConstructableFrom,
};

// Wrapper for [`serenity::model::prelude::Embed`]
#[derive(Clone)]
#[lua_document("TBEmbed", class)]
pub struct TBEmbed(pub Embed);

impl ConstructableFrom<Embed> for TBEmbed {
	fn new(embed: Embed) -> Self {
		TBEmbed(embed)
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

        // methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
        //     Ok(match value.as_str() {
        //         "author" => convert_type_option::<String>(this.0.author.clone(), ctx)?
        //     })
        // });
    }
}
