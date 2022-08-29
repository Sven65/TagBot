use serenity::{model::prelude::{interaction::{application_command::{CommandDataOptionValue, ApplicationCommandInteraction}}, command::CommandOptionType}, builder::CreateApplicationCommand, prelude::Context};

use crate::{util::command_options::*, services::rethinkdb::{tags::{TagsTable}}, tags::legacy::executor::execute_tag};


pub async fn tag(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let name = interaction.data.find_option("name")
		.expect("Expected name option")
		.resolved
		.as_ref()
		.expect("Expected name value");

	let name: String = match name {
		CommandDataOptionValue::String(option) => {option.to_string()}
		&_ => { "Invalid name".to_string() }
	};
	
	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_err() {
		return format!("That tag doesn't exist!");
	} else {
		// Execute tag
		let data = execute_tag(gotten_tag.unwrap(), interaction, ctx).await;

		return data;

		//return format!("Executing tag {}", gotten_tag.unwrap().id);
	}
}


pub fn tag_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
		.required(true)
	});

	let data = command.create_option(|option| {
		option.name("args")
			.kind(CommandOptionType::String)
			.description("Arguments for the tag")
	});

	return data;
}
