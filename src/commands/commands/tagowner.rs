use serenity::{
	builder::{CreateCommand, CreateCommandOption},
	model::prelude::{CommandDataOptionValue, CommandInteraction, CommandOptionType, UserId},
	prelude::Context,
};

use crate::{services::rethinkdb::tags::TagsTable, util::command_options::*};

pub async fn tagowner(interaction: CommandInteraction, ctx: Context) -> String {
	let data = interaction.data.clone();

	let name = &data
		.find_option("name")
		.expect("Expected name option")
		.value;

	let name: String = match name {
		CommandDataOptionValue::String(option) => option.to_string(),
		&_ => "Invalid name".to_string(),
	};

	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_err() {
		"That tag doesn't exist!".to_string()
	} else {
		let tag = gotten_tag.unwrap();
		let user_id = tag.owner.parse::<UserId>();

		if user_id.is_err() {
			return "Failed to get tag owner, as owner ID could not be parsed.".to_string();
		}

		let user = ctx.http.get_user(user_id.unwrap()).await;

		if user.is_ok() {
			let user = user.unwrap();

			format!(
				"The tag {} is owned by {} ({})",
				name.clone(),
				user.tag(),
				tag.owner
			)
		} else {
			format!(
				"The tag {} is owned by unknown user ({})",
				name.clone(),
				tag.owner
			)
		}
	}
}

pub fn tagowner_options_creator(command: CreateCommand) -> CreateCommand {
	command.add_option(
		CreateCommandOption::new(CommandOptionType::String, "name", "The name of the tag")
			.required(true),
	)
}
