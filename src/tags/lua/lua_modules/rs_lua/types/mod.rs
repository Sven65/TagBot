// Modules that wrap rust types as lua types

use rlua::Context;

/// Methods for allowing a type to be registered to the lua module registry
pub trait Requireable {
	/// The function to be called upon requiring the module in the lua module registry
	fn create_module<'lua>(ctx: Context<'lua>) -> rlua::Value<'lua>;
}

pub mod user;
pub mod member;
pub mod timestamp;