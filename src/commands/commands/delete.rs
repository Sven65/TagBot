use cat_loggr::log_fatal;
use serenity::{
	builder::{CreateCommand, CreateCommandOption},
	model::prelude::{CommandDataOptionValue, CommandInteraction, CommandOptionType},
	prelude::Context,
};

use crate::{services::rethinkdb::tags::TagsTable, util::command_options::FindOption};

pub async fn delete(interaction: CommandInteraction, _ctx: Context) -> String {
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
		return "That tag doesn't exist!".to_string();
	}

	if gotten_tag.unwrap().owner != interaction.user.id.to_string() {
		return "You don't own that tag".to_string();
	}

	let res = TagsTable::delete_tag(name.clone()).await;

	if res.is_err() {
		log_fatal!("Failed to delete tag: {:?}", res.err());
		return format!("Failed to delete tag {}", name);
	}

	// if res.is_err()

	format!("Deleted tag {}", name)
}

pub fn delete_options_creator(command: CreateCommand) -> CreateCommand {
	command.add_option(
		CreateCommandOption::new(CommandOptionType::String, "name", "The name of the tag")
			.required(true),
	)
}
