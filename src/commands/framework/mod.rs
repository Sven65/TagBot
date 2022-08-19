pub mod structures;

use lazy_static::{lazy_static};
use serenity::futures::lock::Mutex;
use serenity::model::prelude::interaction::application_command::CommandData;
use serenity::model::prelude::{CommandId};
use serenity::model::prelude::command::{Command};
use serenity::prelude::Context;

use std::collections::HashMap;

use self::structures::OptionCreatorFn;

pub struct CommandIndex {
	pub commands: HashMap<String, fn(CommandData) -> String>,

	context: Option<Context>,
}

impl CommandIndex {
	#[allow(dead_code)]
	pub fn set_ctx(&mut self, ctx: Context) {
		self.context = Some(ctx);
	}

	pub async fn register_command (
		&mut self,
		name: &str,
		f: fn(CommandData) -> String,
		desc: Option<&str>,
		option_creator: Option<OptionCreatorFn>,
	) {
		if let None = self.context {
			panic!("[CommandIndex] Unable to register commands: Context is None.");
		}

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

		println!("Created global command {:#?}", created);

		self.commands.insert(name.to_string(), f);

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
