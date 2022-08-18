pub mod structures;

use structures;

use lazy_static::{lazy_static};
use std::sync::Mutex;

pub struct CommandIndex {
	pub commands: Vec<fn()>,
}

impl CommandIndex {
	pub fn new() -> Self {
		CommandIndex {
			commands: Vec::new()
		}
	}
}

lazy_static! {
	pub static ref COMMAND_INDEX: Mutex<CommandIndex> = Mutex::new(CommandIndex {
		commands: Vec::new(),
	});
}


pub fn register_command(f: fn()) {
	println!("Registering hook.");


	COMMAND_INDEX.lock().unwrap().commands.push(f);
}


#[macro_export]
macro_rules! register_command_macro {
	($item:expr) => { $crate::commands::framework::register_command($item); };
}



// macro_rules! print_message {
// 	() => {
// 		println!("Hello, World!");
// 		crate
// 	};
// }

// print_message!();