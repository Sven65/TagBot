use cat_loggr::log_fatal;
use futures::AsyncWriteExt;
use serenity::{
	builder::{
		CreateAttachment, CreateCommand, CreateCommandOption, CreateInteractionResponse,
		CreateInteractionResponseMessage,
	},
	model::prelude::{CommandDataOptionValue, CommandInteraction, CommandOptionType},
	prelude::Context,
};

use crate::{
	handle_error,
	services::rethinkdb::tags::TagsTable,
	util::{command_options::*, message::send_app_interaction_message},
};

pub async fn raw(interaction: CommandInteraction, ctx: Context) -> String {
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
		handle_error!(
			send_app_interaction_message(ctx, interaction, "That tag doesn't exist", false).await,
			"Failed sending invalid raw tag message"
		);

		return "That tag doesn't exist!".to_string();
	} else {
		let tag = gotten_tag.unwrap();

		let mut file_data = Vec::new();

		handle_error!(
			file_data.write_all(tag.content.as_bytes()).await,
			"Failed to write to temp file while creating raw tag file"
		);

		let file = CreateAttachment::bytes(file_data, format!("{}.txt", tag.id));

		let response = CreateInteractionResponse::Message(
			CreateInteractionResponseMessage::new()
				.content(format!("Raw data for tag {}", tag.id))
				.add_file(file),
		);

		let result = interaction.create_response(&ctx.http, response).await;

		if result.is_err() {
			log_fatal!("Failed sending response {}", result.err().unwrap());
		}
	}

	"".to_string()
}

pub fn raw_options_creator(command: CreateCommand) -> CreateCommand {
	command.add_option(
		CreateCommandOption::new(CommandOptionType::String, "name", "The name of the tag")
			.required(true),
	)
}
