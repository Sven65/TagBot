use serenity::{
	builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
	model::prelude::{CommandInteraction, ModalInteraction},
	prelude::Context,
	Error as SerenityError,
};

pub async fn send_modal_message(
	ctx: Context,
	interaction: ModalInteraction,
	content: &str,
	ephemeral: bool,
) -> Result<(), SerenityError> {
	let response = CreateInteractionResponse::Message(
		CreateInteractionResponseMessage::new()
			.content(content)
			.ephemeral(ephemeral),
	);

	interaction.create_response(&ctx.http, response).await
}

pub async fn send_app_interaction_message(
	ctx: Context,
	interaction: CommandInteraction,
	content: &str,
	ephemeral: bool,
) -> Result<(), SerenityError> {
	let response = CreateInteractionResponse::Message(
		CreateInteractionResponseMessage::new()
			.content(content)
			.ephemeral(ephemeral),
	);

	interaction.create_response(&ctx.http, response).await
}
