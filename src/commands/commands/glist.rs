use serenity::{model::prelude::{interaction::{application_command::{ApplicationCommandInteraction}, InteractionResponseType}, AttachmentType}, prelude::Context};
use tokio::fs::OpenOptions;
use std::{io::{Read, Write}, borrow::Cow};

use crate::services::rethinkdb::tags::TagsTable;

pub async fn glist(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let tags = TagsTable::get_all().await;

	if tags.is_ok() {
		let tags = tags.unwrap();

		let mut file_data = Vec::new();

		for tag in tags.iter() {
			println!("Tag name {}", tag.id);
			file_data.write_all(format!("{}\n", tag.id).as_bytes());
		}
		
		let file = AttachmentType::Bytes { data: file_data.into(), filename: "tags.txt".to_string() };


		let result = interaction.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| {
					message.content(format!("Found {} tags", tags.len()))
						.add_file(file)
			})
		})
		.await;

		if result.is_err() {
			println!("Failed sending response {}", result.err().unwrap());
		}
	} else {
		println!("Failed to get tags {}", tags.err().unwrap());

		let result = interaction.create_interaction_response(&ctx.http, |response| {
			response.kind(InteractionResponseType::ChannelMessageWithSource)
			.interaction_response_data(|message| {
				message.content("Failed to list tags")
			})
		}).await;

		if result.is_err() {
			println!("Failed sending response {}", result.err().unwrap());
		}
	}


	return "".to_string();
}