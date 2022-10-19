pub mod structures;

use cat_loggr::log_debug;
use lazy_static::lazy_static;
// use serenity::futures::lock::Mutex;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::CommandId;
use serenity::prelude::Context;
// use futures::Future;
use tokio::sync::Mutex;

use core::fmt::Debug;
use std::collections::HashMap;

use crate::handle_error;

use self::structures::{
	CommandComponentHandlerFn, CommandExecutorFn, CommandModalHandlerFn, OptionCreatorFn,
};

const CREATE_COMMANDS: bool = false;

#[derive(Debug, Clone, Copy)]
pub struct TBCommand {
	pub executor: CommandExecutorFn,
	pub sends_message: bool,
	pub modal_handler: Option<CommandModalHandlerFn>,
	pub component_handler: Option<CommandComponentHandlerFn>,
}

#[derive(Clone)]
pub struct CommandIndex {
	pub commands: HashMap<String, TBCommand>,

	context: Option<Context>,
}

pub struct RegistrationMeta {
	pub f: CommandExecutorFn,
	pub desc: Option<String>,
	pub option_creator: Option<OptionCreatorFn>,
	pub sends_message: bool,
	pub modal_handler: Option<CommandModalHandlerFn>,
	pub component_handler: Option<CommandComponentHandlerFn>,
}

impl Debug for CommandIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CommandIndex")
			.field("commands", &self.commands)
			.finish()
	}
}

impl CommandIndex {
	#[allow(dead_code)]
	pub fn set_ctx(&mut self, ctx: Context) {
		self.context = Some(ctx);
	}

	pub async fn register_command(&mut self, name: &str, options: RegistrationMeta) {
		if self.context.is_none() {
			panic!("[CommandIndex] Unable to register commands: Context is None.");
		}

		if CREATE_COMMANDS {
			let created = Command::create_global_application_command(
				&self.context.as_ref().unwrap().http,
				|command| {
					command.name(name).description(
						options
							.desc
							.unwrap_or_else(|| "Default description.".to_string()),
					);

					match options.option_creator {
						Some(option_creator) => option_creator(command),
						None => command,
					}
				},
			)
			.await;

			log_debug!("Created global command {:#?}", created.unwrap().name);
		}

		let tb_command = TBCommand {
			executor: options.f,
			sends_message: options.sends_message,
			modal_handler: options.modal_handler,
			component_handler: options.component_handler,
		};

		self.commands.insert(name.to_string(), tb_command);

		log_debug!("Registered to index");
	}

	#[allow(dead_code)]
	pub async fn remove_command(&mut self, id: CommandId) {
		if self.context.is_none() {
			panic!("[CommandIndex] Unable to remove commands: Context is None.");
		}

		handle_error!(
			Command::delete_global_application_command(&self.context.as_ref().unwrap().http, id)
				.await,
			"Failed to delete command"
		);
	}
}

lazy_static! {
	pub static ref COMMAND_INDEX: Mutex<CommandIndex> =
		Mutex::new(CommandIndex { commands: HashMap::new(), context: None });
}
