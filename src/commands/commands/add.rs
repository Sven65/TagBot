use serenity::{model::prelude::{interaction::application_command::{CommandData, CommandDataOptionValue}, command::CommandOptionType}, builder::CreateApplicationCommand};

use crate::util::command_options::*;

pub fn add(data: CommandData) -> String {
	let name = data.find_option("name")
		.expect("Expected name option")
		.resolved
		.as_ref()
		.expect("Expected name value");

	let contents = data.find_option("content")
		.expect("Expected content option")
		.resolved
		.as_ref()
		.expect("Expected content value");
		

	let name: String = match name {
		CommandDataOptionValue::String(option) => {option.to_string()}
		&_ => { "Invalid name".to_string() }
	};

	let contents = match contents {
		CommandDataOptionValue::String(option) => {option.to_string()}
		&_ => { "Invalid contents".to_string() }
	};



	format!("Creating tag {} with contents {}", name, contents)

}


pub fn add_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
		.required(true)
	});

	let data = command.create_option(|option| {
		option.name("content")
		.kind(CommandOptionType::String)
		.description("The contents of the tag")
		.required(true)
	});

	return data;
}
