use serenity::{model::prelude::interaction::{modal::ModalSubmitInteraction, InteractionResponseType}, prelude::Context, Error as SerenityError};



pub async fn send_modal_message (ctx: Context, interaction: ModalSubmitInteraction, content: &str, ephemeral: bool) -> Result<(), SerenityError> {
	return interaction.create_interaction_response(&ctx.http, |response| {
		response
			.kind(InteractionResponseType::ChannelMessageWithSource)
			.interaction_response_data(|message| {
				message
					.content(content)
					.ephemeral(ephemeral)
		})
	}).await;
}