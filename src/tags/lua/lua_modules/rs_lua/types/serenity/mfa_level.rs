use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::guild::MfaLevel;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;


/// Wrapper for [`serenity::model::guild::MfaLevel`]
#[derive(Clone)]
pub struct TBMfaLevel(pub MfaLevel);

impl ConstructableFrom<MfaLevel> for TBMfaLevel {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `mfa_level` - The serenity MfaLevel to wrap
	fn new(mfa_level: MfaLevel) -> TBMfaLevel {
		TBMfaLevel(mfa_level)
	}
}

impl UserData for TBMfaLevel {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			let level = match this.0 {
				MfaLevel::None => "None",
				MfaLevel::Elevated => "Elevated",
				MfaLevel::Unknown => "Unknown",
				_ => "Unknown",
			};

			Ok(level.to_lua(ctx)?)
		});
	}
}