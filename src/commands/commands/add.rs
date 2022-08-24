use reql::{r};
use serenity::{model::prelude::{interaction::application_command::{CommandDataOptionValue, ApplicationCommandInteraction}, command::CommandOptionType}, builder::CreateApplicationCommand, prelude::Context};

use crate::{util::command_options::*, services::rethinkdb::{tags::{TagsTable}}};

pub async fn add(interaction: ApplicationCommandInteraction, _ctx: Context) -> String {
	let data = interaction.data.clone();

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

	// let exists = TagsTable::tag_exists()

	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_ok() {
		return format!("That tag already exists!");
	}

	let result = TagsTable::add_tag(name.clone(), contents, interaction.user.id.to_string()).await;

	if result.is_ok() {
		return format!("Added tag {}", name);
	} else {
		println!("Error adding tag: {:?}", result.err());
		return "Error while adding tag".to_string();
	}
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
