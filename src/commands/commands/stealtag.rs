use futures::AsyncWriteExt;
use reql::{r};
use serenity::{model::prelude::{interaction::{application_command::{CommandDataOptionValue, ApplicationCommandInteraction}, InteractionResponseType}, command::CommandOptionType, AttachmentType}, builder::CreateApplicationCommand, prelude::Context};

use crate::{util::command_options::*, services::rethinkdb::{tags::{TagsTable}}};


pub async fn stealtag(interaction: ApplicationCommandInteraction, _ctx: Context) -> String {
	let allowed_users: [String; 1] = ["141610251299454976".to_owned()];


	let data = interaction.data.clone();

	if !allowed_users.contains(&interaction.user.id.to_string()) {
		return format!("You're not allowed to execute this command.");
	}

	let name = data.find_option("name")
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
		let update_result = TagsTable::set_owner(name.clone(), interaction.user.id.to_string()).await;

		if update_result.is_ok() {
			return format!("Stole the tag {}", name);
		} else {
			return format!("Failed to steal tag.");
		}
	}
}


pub fn stealtag_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	let data = command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
		.required(true)
	});

	return data;
}
