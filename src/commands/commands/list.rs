use cat_loggr::log_fatal;
use serenity::{
	builder::{CreateAttachment, CreateInteractionResponse, CreateInteractionResponseMessage},
	model::prelude::CommandInteraction,
	prelude::Context,
};
use std::io::Write;

use crate::{handle_error, services::rethinkdb::tags::TagsTable};

pub async fn list(interaction: CommandInteraction, ctx: Context) -> String {
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

		let file = CreateAttachment::bytes(file_data, "tags.txt");

		let response = CreateInteractionResponse::Message(
			CreateInteractionResponseMessage::new()
				.content(format!("Found {} tags", tags.len()))
				.add_file(file),
		);

		let result = interaction.create_response(&ctx.http, response).await;

		if result.is_err() {
			log_fatal!("Failed sending response {}", result.err().unwrap());
		}
	} else {
		log_fatal!("Failed to get tags {}", tags.err().unwrap());

		let response = CreateInteractionResponse::Message(
			CreateInteractionResponseMessage::new().content("Failed to list tags"),
		);

		let result = interaction.create_response(&ctx.http, response).await;

		if result.is_err() {
			log_fatal!("Failed sending response {}", result.err().unwrap());
		}
	}

	"".to_string()
}
