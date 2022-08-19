use serenity::model::prelude::interaction::application_command::{CommandData, CommandDataOption};


pub trait FindOption {
	fn find_option(&self, name: &str) -> Option<&CommandDataOption>;
}

impl FindOption for CommandData {
    fn find_option(&self, name: &str) -> Option<&CommandDataOption> {
		return self.options.iter().find(|item| {
			return item.name == name;
		})
    }
}