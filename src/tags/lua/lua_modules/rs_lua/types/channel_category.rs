use serenity::model::prelude::ChannelCategory;
use rlua::{UserData, MetaMethod, Value, ToLua, Error as LuaError};
use tagbot_macro::ud_index;

/// Wrapper for a Serenity Channel
#[derive(Clone, Debug)]
pub struct TBChannelCategory(ChannelCategory);

impl TBChannelCategory {
	/// Creates a new wrapper
	pub fn new(category: ChannelCategory) -> TBChannelCategory {
		TBChannelCategory(category)
	}
}


#[ud_index("name", AccessType::Field, "name", LuaType::Value)]
impl UserData for TBChannelCategory {
	// fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
	// 	methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
	// 		Ok(match &value.as_str() {
	// 			&"c" => this.0.name
	// 		})
	// 	})
	// }

}
