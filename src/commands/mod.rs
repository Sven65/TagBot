use crate::register_command_macro;

use self::ping2::ping2;

pub mod framework;
pub mod ping2;

pub fn initCommands() {
	register_command_macro!(ping2);
}