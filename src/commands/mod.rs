// use crate::{register_command_macro, commands::framework::register_command};
pub mod framework;
#[macro_use]
pub mod commands;

use commands::ping2::ping2;

use self::framework::COMMAND_INDEX;

#[allow(dead_code)]
pub async fn init_commands() {
	COMMAND_INDEX.lock().await.register_command("ping2", ping2, Some("With desc")).await;
}
