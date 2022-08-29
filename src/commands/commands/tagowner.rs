use serenity::{model::prelude::{interaction::{application_command::{CommandDataOptionValue, ApplicationCommandInteraction}}, command::CommandOptionType}, builder::CreateApplicationCommand, prelude::Context};

use crate::{util::command_options::*, services::rethinkdb::{tags::{TagsTable}}};


pub async fn tagowner(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let data = interaction.data.clone();

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
		let tag = gotten_tag.unwrap();
		let user_id = tag.owner.parse::<u64>();

		if user_id.is_err() {
			return format!("Failed to get tag owner, as owner ID could not be parsed.");
		}

		let user = ctx.http.get_user(user_id.unwrap()).await;

		if user.is_ok() {
			let user = user.unwrap();

			return format!("The tag {} is owned by {} ({})", name.clone(), user.tag(), tag.owner);
		} else {
			return format!("The tag {} is owned by unknown user ({})", name.clone(), tag.owner);
		}
	}
}


pub fn tagowner_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	let data = command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
		.required(true)
	});

	return data;
}
