use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		command::CommandOptionType,
		component::InputTextStyle,
		interaction::{
			application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
			modal::ModalSubmitInteraction,
			InteractionResponseType,
		},
	},
	prelude::Context,
};

use crate::{
	handle_error,
	services::rethinkdb::tags::TagsTable,
	util::{command_options::*, input_field::FindInput, message::send_modal_message},
};

pub async fn edit(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let name = interaction
		.data
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
		return "That tag doesn't exist!".to_string();
	}

	let gotten_tag = gotten_tag.unwrap();

	if gotten_tag.owner != interaction.user.id.to_string() {
		return "You don't own that tag".to_string();
	}

	let modal = interaction
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::Modal)
				.interaction_response_data(|modal| {
					modal
						.custom_id(format!("edit-{}-{}", interaction.user.id, name))
						.title(format!("Editing tag {}", name))
						.components(|comp| {
							comp.create_action_row(|row| {
								row.create_input_text(|input| {
									input
										.custom_id("content_input")
										.label("Content")
										.min_length(1)
										.required(true)
										.style(InputTextStyle::Paragraph)
										.value(gotten_tag.content)
								})
							})
						})
				})
		})
		.await;

	if modal.is_err() {
		println!("Failed to create modal: {}", modal.err().unwrap())
	}

	"".to_string()
}

pub async fn edit_tag_handle_modal(interaction: ModalSubmitInteraction, ctx: Context) -> String {
	let mut id_split = interaction.data.custom_id.split('-');
	let tag_name = id_split.nth(2);

	if tag_name.is_none() {
		return "Failed to parse tag name from modal ID".to_string();
	}

	let content_field = interaction.data.components.find_input("content_input");
	let content = &content_field.unwrap().value;

	if content.is_empty() {
		handle_error!(
			send_modal_message(ctx, interaction, "Expected content to be provided.", true).await,
			"Failed to send empty content modal error"
		);
		return "".to_string();
	}

	let name = tag_name.unwrap().to_string();

	let result = TagsTable::set_content(name.clone(), content.clone()).await;

	if result.is_ok() {
		handle_error!(
			send_modal_message(
				ctx,
				interaction.clone(),
				&format!("Edited tag {}", name.clone()),
				false
			)
			.await,
			"Failed to send tag edit success message"
		);
		"".to_string()
	} else {
		println!("Error editing tag: {:?}", result.err());
		handle_error!(
			send_modal_message(ctx, interaction.clone(), "Error while editing tag", false).await,
			"Failed to send tag edit error message"
		);

		"".to_string()
	}
}

pub fn edit_options_creator(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command.create_option(|option| {
		option
			.name("name")
			.kind(CommandOptionType::String)
			.description("The name of the tag to edit")
			.required(true)
	}) as _
}
