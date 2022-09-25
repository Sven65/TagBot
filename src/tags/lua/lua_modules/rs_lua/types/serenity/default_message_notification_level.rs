use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::guild::DefaultMessageNotificationLevel;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;


/// Wrapper for [`serenity::model::guild::DefaultMessageNotificationLevel`]
#[derive(Clone)]
pub struct TBDefaultMessageNotificationLevel(pub DefaultMessageNotificationLevel);

impl ConstructableFrom<DefaultMessageNotificationLevel> for TBDefaultMessageNotificationLevel {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `notification_level` - The serenity DefaultMessageNotificationLevel to wrap
	fn new(notification_level: DefaultMessageNotificationLevel) -> TBDefaultMessageNotificationLevel {
		TBDefaultMessageNotificationLevel(notification_level)
	}
}

impl UserData for TBDefaultMessageNotificationLevel {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			let quality = match this.0 {
				DefaultMessageNotificationLevel::All => "All",
				DefaultMessageNotificationLevel::Mentions => "Mentions",
				DefaultMessageNotificationLevel::Unknown => "Unknown",
				_ => "Unknown",
			};

			Ok(quality.to_lua(ctx)?)
		});
	}
}