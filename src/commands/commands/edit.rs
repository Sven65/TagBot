use serenity::{model::prelude::{interaction::{application_command::{CommandDataOptionValue, ApplicationCommandInteraction}, InteractionResponseType, modal::ModalSubmitInteraction}, command::CommandOptionType, component::InputTextStyle}, builder::CreateApplicationCommand, prelude::Context};

use crate::{util::{command_options::*, input_field::FindInput, message::send_modal_message}, services::rethinkdb::{tags::{TagsTable}}, handle_error};

// pub async fn edit(interaction: ApplicationCommandInteraction, _ctx: Context) -> String {
// 	let data = interaction.data.clone();

// 	let name = data.find_option("name")
// 		.expect("Expected name option")
// 		.resolved
// 		.as_ref()
// 		.expect("Expected name value");

// 	let contents = data.find_option("content")
// 		.expect("Expected content option")
// 		.resolved
// 		.as_ref()
// 		.expect("Expected content value");
		

// 	let name: String = match name {
// 		CommandDataOptionValue::String(option) => {option.to_string()}
// 		&_ => { "Invalid name".to_string() }
// 	};

// 	let contents = match contents {
// 		CommandDataOptionValue::String(option) => {option.to_string()}
// 		&_ => { "Invalid contents".to_string() }
// 	};

// 	let gotten_tag = TagsTable::get_tag(name.clone()).await;

// 	if gotten_tag.is_err() {
// 		return format!("That tag doesn't exist!");
// 	}

// 	if gotten_tag.unwrap().owner != interaction.user.id.to_string() {
// 		return format!("You don't own that tag");
// 	}

// 	let result = TagsTable::edit_tag(name.clone(), contents, interaction.user.id.to_string()).await;

// 	if result.is_ok() {
// 		return format!("Edited tag {}", name);
// 	} else {
// 		println!("Error editing tag: {:?}", result.err());
// 		return "Error while editing tag".to_string();
// 	}
// }

pub async fn edit(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	
	let name = interaction.data.find_option("name")
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
	}

	let gotten_tag = gotten_tag.unwrap();

	if gotten_tag.owner != interaction.user.id.to_string() {
		return format!("You don't own that tag");
	}


	let modal = interaction.create_interaction_response(&ctx.http, |response| {
		response.kind(InteractionResponseType::Modal)
		.interaction_response_data(|modal| {
			modal.custom_id(format!("edit-{}-{}", interaction.user.id, name))
				.title(format!("Editing tag {}", name))
				.components(|comp| {
					comp.create_action_row(|row| {
						row.create_input_text(|input| {
							input.custom_id("content_input")
								.label("Content")
								.min_length(1)
								.required(true)
								.style(InputTextStyle::Paragraph)	
								.value(gotten_tag.content)
						})
					})
				})
		})
	}).await;

	if modal.is_err() {
		println!("Failed to create modal: {}", modal.err().unwrap())
	}

	return "".to_string();
}

pub async fn edit_tag_handle_modal (interaction: ModalSubmitInteraction, ctx: Context) -> String {
	let mut id_split = interaction.data.custom_id.split("-");
	let tag_name = id_split.nth(2);

	if tag_name.is_none() {
		return "Failed to parse tag name from modal ID".to_string();
	}
	
	let content_field = interaction.data.components.find_input("content_input");
	let content = &content_field.unwrap().value;

	if content.is_empty() {
		handle_error!(send_modal_message(ctx, interaction, "Expected content to be provided.", true).await, "Failed to send empty content modal error");
		return "".to_string();
	}

	let name = tag_name.unwrap().clone().to_string();

	let result = TagsTable::set_content(name.clone(), content.clone()).await;

	if result.is_ok() {
		handle_error!(send_modal_message(ctx, interaction.clone(), &format!("Edited tag {}", name.clone()), false).await, "Failed to send tag edit success message");
		return "".to_string();
	} else {
		println!("Error editing tag: {:?}", result.err());
		handle_error!(send_modal_message(ctx, interaction.clone(), "Error while editing tag", false).await, "Failed to send tag edit error message");

		return "".to_string();
	}
	
}

pub fn edit_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	let data = command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag to edit")
		.required(true)
	});

	return data;
}
