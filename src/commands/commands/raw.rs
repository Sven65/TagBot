use cat_loggr::log_fatal;
use futures::AsyncWriteExt;
use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		command::CommandOptionType,
		interaction::{
			application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
			InteractionResponseType,
		},
		AttachmentType,
	},
	prelude::Context,
};

use crate::{
	handle_error,
	services::rethinkdb::tags::TagsTable,
	util::{command_options::*, message::send_app_interaction_message},
};

pub async fn raw(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let data = interaction.data.clone();

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

		let file =
			AttachmentType::Bytes { data: file_data.into(), filename: format!("{}.txt", tag.id) };

		let result = interaction
			.create_interaction_response(&ctx.http, |response| {
				response
					.kind(InteractionResponseType::ChannelMessageWithSource)
					.interaction_response_data(|message| {
						message
							.content(format!("Raw data for tag {}", tag.id))
							.add_file(file)
					})
			})
			.await;

		if result.is_err() {
			log_fatal!("Failed sending response {}", result.err().unwrap());
		}
	}

	"".to_string()
}

pub fn raw_options_creator(
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
