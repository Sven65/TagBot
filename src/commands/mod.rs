// use crate::{register_command_macro, commands::framework::register_command};
use std::{future::Future, pin::Pin};

pub mod framework;
#[macro_use]
pub mod commands;

use commands::ping2::ping2;
use commands::add::add;

use self::commands::add::add_options_creator;
use self::framework::{COMMAND_INDEX, CommandIndex};


#[allow(dead_code)]
pub async fn init_commands() {
	COMMAND_INDEX.lock().await.register_command("ping2", |data| Box::pin(ping2(data)), Some("With desc"), None).await;
	COMMAND_INDEX.lock().await.register_command("add", |data| Box::pin(add(data)), Some("Adds tags"), Some(add_options_creator)).await;
}
