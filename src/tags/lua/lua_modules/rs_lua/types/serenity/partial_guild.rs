use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::{model::guild::PartialGuild, prelude::{Context as SerenityContext}};

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;


/// Wrapper for [`serenity::model::guild::PartialGuild`]
#[derive(Clone)]
pub struct TBPartialGuild(pub PartialGuild, pub SerenityContext);

impl ConstructableFrom2<PartialGuild, SerenityContext> for TBPartialGuild {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `partial_guild` - The serenity PartialGuild to wrap
	/// * `context` - The serenity context to wrap
	fn new(partial_guild: PartialGuild, context: SerenityContext) -> TBPartialGuild {
		TBPartialGuild(partial_guild, context)
	}
}

impl UserData for TBPartialGuild {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0. .to_lua(ctx)?)
		// });

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&""
			})
		})

	}
}