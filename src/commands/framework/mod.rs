pub mod structures;

use lazy_static::{lazy_static};
// use serenity::futures::lock::Mutex;
use serenity::model::prelude::{CommandId};
use serenity::model::prelude::command::{Command};
use serenity::prelude::Context;
// use futures::Future;
use tokio::sync::Mutex;

use std::collections::HashMap;
use core::fmt::Debug;

use self::structures::{OptionCreatorFn, CommandExecutorFn, CommandModalHandlerFn};

const CREATE_COMMANDS: bool = false;

#[derive(Debug, Clone, Copy)]
pub struct TBCommand {
	pub executor: CommandExecutorFn,
	pub sends_message: bool,
	pub modal_handler: Option<CommandModalHandlerFn>,
}

#[derive(Clone)]
pub struct CommandIndex {
	pub commands: HashMap<String, TBCommand>,

	context: Option<Context>,
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

	pub async fn register_command (
		&mut self,
		name: &str,
		f: CommandExecutorFn,
		desc: Option<&str>,
		option_creator: Option<OptionCreatorFn>,
		sends_message: bool,
		modal_handler: Option<CommandModalHandlerFn>,
	) {
		if let None = self.context {
			panic!("[CommandIndex] Unable to register commands: Context is None.");
		}

		if CREATE_COMMANDS {
			let created = Command::create_global_application_command(&self.context.as_ref().unwrap().http, |command| {
				command.name(name)
					.description(desc.unwrap_or("Default description."));

					let data = match option_creator {
						Some(option_creator) => option_creator(command),
						None => { command },
					};

					data
					
			})
			.await;

			println!("Created global command {:#?}", created.unwrap().name);
		}

		let tb_command = TBCommand {
			executor: f,
			sends_message,
			modal_handler,
		};

		self.commands.insert(name.to_string(), tb_command);

		dbg!("Registered to index");
	}

	#[allow(dead_code)]
	pub async fn remove_command(&mut self, id: CommandId) {
		if let None = self.context {
			panic!("[CommandIndex] Unable to remove commands: Context is None.");
		}

		let deleted = Command::delete_global_application_command(&self.context.as_ref().unwrap().http, id).await;

		println!("Delete result: {:#?}", deleted);
	}
}

lazy_static! {
	pub static ref COMMAND_INDEX: Mutex<CommandIndex> = Mutex::new(CommandIndex {
		commands: HashMap::new(),
		context: None,
	});
}
