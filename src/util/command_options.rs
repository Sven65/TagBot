use serenity::model::prelude::interaction::application_command::{CommandData, CommandDataOption};


pub trait FindOption {
	fn find_option(&self, name: &str) -> Option<&CommandDataOption>;
}

impl FindOption for CommandData {
	/// Finds a command option by name
	/// 
	/// # Arguments
	/// 
	/// * `name` - The name of the option to find
    fn find_option(&self, name: &str) -> Option<&CommandDataOption> {
		return self.options.iter().find(|item| {
			return item.name == name;
		})
    }
}