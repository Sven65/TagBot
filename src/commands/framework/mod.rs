pub mod structures;

use lazy_static::{lazy_static};
use serenity::futures::lock::Mutex;
use serenity::model::prelude::command::Command;
use serenity::prelude::Context;

use std::collections::HashMap;

pub struct CommandIndex {
	pub commands: HashMap<String, fn() -> String>,

	context: Option<Context>,
}

impl CommandIndex {
	// pub fn new() -> Self {
	// 	CommandIndex {
	// 		commands: HashMap::new(),
	// 		context: None,
	// 	}
	// }

	#[allow(dead_code)]
	pub fn set_ctx(&mut self, ctx: Context) {
		self.context = Some(ctx);
	}

	pub async fn register_command (&mut self, name: &str, f: fn() -> String, desc: Option<&str>) {
		if let None = self.context {
			panic!("[CommandIndex] Unable to register commands: Context is None.");
		}

		let created = Command::create_global_application_command(&self.context.as_ref().unwrap().http, |command| {
			command.name(name)
				.description(desc.unwrap_or("Default description."))
		})
		.await;

		println!("Created global command {:#?}", created);

		self.commands.insert(name.to_string(), f);

		dbg!("Registered to index");
	}
}

lazy_static! {
	pub static ref COMMAND_INDEX: Mutex<CommandIndex> = Mutex::new(CommandIndex {
		commands: HashMap::new(),
		context: None,
	});
}
