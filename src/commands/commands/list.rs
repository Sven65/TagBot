use serenity::{
	model::prelude::{
		interaction::{
			application_command::ApplicationCommandInteraction, InteractionResponseType,
		},
		AttachmentType,
	},
	prelude::Context,
};
use std::io::Write;

use crate::{handle_error, services::rethinkdb::tags::TagsTable};

pub async fn list(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let tags = TagsTable::get_all_by_owner(interaction.user.id.to_string()).await;

	if tags.is_ok() {
		let tags = tags.unwrap();

		let mut file_data = Vec::new();

		for tag in tags.iter() {
			handle_error!(
				file_data.write_all(format!("{}\n", tag.id).as_bytes()),
				"Failed to write to temp file while creating user list"
			);
		}

		let file =
			AttachmentType::Bytes { data: file_data.into(), filename: "tags.txt".to_string() };

		let result = interaction
			.create_interaction_response(&ctx.http, |response| {
				response
					.kind(InteractionResponseType::ChannelMessageWithSource)
					.interaction_response_data(|message| {
						message
							.content(format!("Found {} tags", tags.len()))
							.add_file(file)
					})
			})
			.await;

		if result.is_err() {
			println!("Failed sending response {}", result.err().unwrap());
		}
	} else {
		println!("Failed to get tags {}", tags.err().unwrap());

		let result = interaction
			.create_interaction_response(&ctx.http, |response| {
				response
					.kind(InteractionResponseType::ChannelMessageWithSource)
					.interaction_response_data(|message| message.content("Failed to list tags"))
			})
			.await;

		if result.is_err() {
			println!("Failed sending response {}", result.err().unwrap());
		}
	}

	"".to_string()
}
