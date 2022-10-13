use serenity::{
	model::prelude::interaction::{
		application_command::ApplicationCommandInteraction, modal::ModalSubmitInteraction,
		InteractionResponseType,
	},
	prelude::Context,
	Error as SerenityError,
};

pub async fn send_modal_message(
	ctx: Context,
	interaction: ModalSubmitInteraction,
	content: &str,
	ephemeral: bool,
) -> Result<(), SerenityError> {
	interaction
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| message.content(content).ephemeral(ephemeral))
		})
		.await
}

pub async fn send_app_interaction_message(
	ctx: Context,
	interaction: ApplicationCommandInteraction,
	content: &str,
	ephemeral: bool,
) -> Result<(), SerenityError> {
	interaction
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| message.content(content).ephemeral(ephemeral))
		})
		.await
}
