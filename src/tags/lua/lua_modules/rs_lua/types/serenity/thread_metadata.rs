use rlua::UserData;
use serenity::model::channel::ThreadMetadata;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;

/// Wrapper for [`serenity::model::channel::ThreadMetadata`]
#[derive(Clone)]
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
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0.to_string().to_lua(ctx)?)
		// });

		// methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
		// 	Ok(match &value.as_str() {

		// 	})
		// })
	}
}
