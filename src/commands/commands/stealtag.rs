use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		command::CommandOptionType,
		interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
	},
	prelude::Context,
};

use crate::{services::rethinkdb::tags::TagsTable, util::command_options::*};

pub async fn stealtag(interaction: ApplicationCommandInteraction, _ctx: Context) -> String {
	let allowed_users: [String; 1] = ["141610251299454976".to_owned()];

	let data = interaction.data.clone();

	if !allowed_users.contains(&interaction.user.id.to_string()) {
		return "You're not allowed to execute this command.".to_string();
	}

	let name = data
		.find_option("name")
		.expect("Expected name option")
		.resolved
		.as_ref()
		.expect("Expected name value");

	let name: String = match name {
		CommandDataOptionValue::String(option) => option.to_string(),
		&_ => "Invalid name".to_string(),
	};

	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_err() {
		"That tag doesn't exist!".to_string()
	} else {
		let update_result =
			TagsTable::set_owner(name.clone(), interaction.user.id.to_string()).await;

		if update_result.is_ok() {
			format!("Stole the tag {}", name)
		} else {
			"Failed to steal tag.".to_string()
		}
	}
}

pub fn stealtag_options_creator(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command.create_option(|option| {
		option
			.name("name")
			.kind(CommandOptionType::String)
			.description("The name of the tag")
			.required(true)
	}) as _
}
