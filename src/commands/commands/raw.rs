use futures::AsyncWriteExt;
use serenity::{model::prelude::{interaction::{application_command::{CommandDataOptionValue, ApplicationCommandInteraction}, InteractionResponseType}, command::CommandOptionType, AttachmentType}, builder::CreateApplicationCommand, prelude::Context};

use crate::{util::command_options::*, services::rethinkdb::{tags::{TagsTable}}, handle_error};

pub async fn raw(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
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

		let mut file_data = Vec::new();

		handle_error!(file_data.write_all(tag.content.as_bytes()).await, "Failed to write to temp file while creating raw tag file");

		let file = AttachmentType::Bytes { data: file_data.into(), filename: format!("{}.txt", tag.id) };

		let result = interaction.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| {
					message.content(format!("Raw data for tag {}", tag.id))
						.add_file(file)
			})
		})
		.await;

		if result.is_err() {
			println!("Failed sending response {}", result.err().unwrap());
		}
	}

	return "".to_string();

}


pub fn raw_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	let data = command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
		.required(true)
	});

	return data;
}
