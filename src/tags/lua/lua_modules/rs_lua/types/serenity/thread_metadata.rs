use rlua::{MetaMethod, UserData, Value};
use serenity::model::channel::ThreadMetadata;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::utils::{
	functions::{convert_constructable_option, convert_type, convert_type_option},
	types::ConstructableFrom,
};

use super::timestamp::TBTimestamp;

/// Wrapper for [`serenity::model::channel::ThreadMetadata`]
#[derive(Clone)]
#[lua_document("TBThreadMetadata", class)]
pub struct TBThreadMetadata(pub ThreadMetadata);

impl ConstructableFrom<ThreadMetadata> for TBThreadMetadata {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `thread_metadata` - The serenity ThreadMetadata to wrap
	fn new(thread_metadata: ThreadMetadata) -> TBThreadMetadata {
		TBThreadMetadata(thread_metadata)
	}
}

impl UserData for TBThreadMetadata {
	#[rustfmt::skip]
	#[lua_document("TBThreadMetadata", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0.to_string().to_lua(ctx)?)
		// });

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"archived" => convert_type::<bool>(this.0.archived, ctx)?,
				"auto_archive_duration" => convert_type_option::<u64>(this.0.auto_archive_duration, ctx)?,
				"archive_timestamp" => convert_constructable_option::<TBTimestamp, _>(this.0.archive_timestamp, ctx)?,
				"locked" => convert_type::<bool>(this.0.locked, ctx)?,
				"create_timestamp" => convert_constructable_option::<TBTimestamp, _>(this.0.create_timestamp, ctx)?,
				"invitable" => convert_type::<bool>(this.0.invitable, ctx)?,
				_ => Value::Nil,
			})
		})
	}
}
