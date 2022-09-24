use serenity::model::prelude::ChannelCategory;
use rlua::{UserData};

use super::utils::types::ConstructableFrom;

/// Wrapper for a Serenity Channel
#[derive(Clone, Debug)]
pub struct TBChannelCategory(pub ChannelCategory);

impl ConstructableFrom<ChannelCategory> for TBChannelCategory {
    fn new(value: ChannelCategory) -> Self {
        TBChannelCategory(value)
    }
}

impl UserData for TBChannelCategory {
	// fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
	// 	methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
	// 		Ok(match &value.as_str() {
	// 			&"c" => {
	// 				let gotten_value = this.0.name().clone();
    //                 gotten_value.to_lua(ctx)?
	// 			}
	// 		})
	// 	})
	// }

}
