use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::{model::{prelude::VideoQualityMode}};

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;


/// Wrapper for [`serenity::model::id::MessageId`]
#[derive(Clone)]
pub struct TBVideoQualityMode(pub VideoQualityMode);

impl ConstructableFrom<VideoQualityMode> for TBVideoQualityMode {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `video_quality_mode` - The serenity VideoQualityMode to wrap
	fn new(video_quality_mode: VideoQualityMode) -> TBVideoQualityMode {
		TBVideoQualityMode(video_quality_mode)
	}
}

impl UserData for TBVideoQualityMode {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			let quality = match this.0 {
				VideoQualityMode::Auto => "Auto",
				VideoQualityMode::Full => "Full",
				VideoQualityMode::Unknown => "Unknown",
				_ => "Unknown",
			};

			Ok(quality.to_lua(ctx)?)
		});
	}
}