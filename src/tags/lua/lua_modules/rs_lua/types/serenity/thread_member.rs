use rlua::{UserData, MetaMethod, Value, ToLua};
use serenity::model::guild::ThreadMember;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom;


/// Wrapper for [`serenity::model::guild::ThreadMember`]
#[derive(Clone)]
pub struct TBThreadMember(pub ThreadMember);

impl ConstructableFrom<ThreadMember> for TBThreadMember {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `thread_member` - The serenity ThreadMember to wrap
	fn new(thread_member: ThreadMember) -> TBThreadMember {
		TBThreadMember(thread_member)
	}
}

impl UserData for TBThreadMember {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
		// 	Ok(this.0.to_string().to_lua(ctx)?)
		// });

		// methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
		// Ok(match &value.as_str() {

		// })
		//})

	}
}