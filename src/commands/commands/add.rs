use serenity::{model::prelude::{interaction::{application_command::{ApplicationCommandInteraction}, InteractionResponseType, modal::ModalSubmitInteraction, message_component::MessageComponentInteraction}, component::InputTextStyle}, prelude::Context};

use crate::{util::{input_field::FindInput, message::{send_modal_message, send_component_message}}, services::rethinkdb::{tags::{TagsTable, TagType}}, handle_error};

pub async fn add(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	let result = interaction.create_interaction_response(&ctx.http, |response| {
		response.interaction_response_data(|data| {
			data.content("Please select a tag type")
				.components(|components| {
				components.create_action_row(|row| {
					row.create_select_menu(|menu| {
						menu.custom_id(format!("add-{}", interaction.user.id))
							.min_values(1)
							.max_values(1)
							.options(|opts| {
								opts.create_option(|opt| {
									opt.description("A regular tag")
										.label("Regular tag")
										.value("legacy")
								})
								.create_option(|opt| {
									opt.description("A scriptable tag")
										.label("Lua tag")
										.value("lua")
								})
							})
					})
				})
			})
		})
	}).await;

	if result.is_err() {
		println!("Error sending tag type select menu: {:#?}", result.err().unwrap());
	}

	return "".to_string();
}

pub async fn add_tag_handle_component(interaction: MessageComponentInteraction, ctx: Context) -> String {

	let value = interaction.data.values.get(0);

	if value.is_none() {
		handle_error!(send_component_message(ctx, interaction, "Expected tag type to be provided.", true).await, "Failed to send empty tag type select error");
		return "".to_string();
	}

	let modal = interaction.create_interaction_response(&ctx.http, |response| {
		response.kind(InteractionResponseType::Modal)
		.interaction_response_data(|modal| {
			modal.custom_id(format!("add-{}-{}", interaction.user.id, value.unwrap()))
				.title("Create a new tag")
				.components(|comp| {
					

					comp.create_action_row(|row| {
						row.create_input_text(|input| {
							input.custom_id("name_input")
								.label("Name")
								.min_length(1)
								.required(true)
								.style(InputTextStyle::Short)	
						})
					})
					
					.create_action_row(|row| {
						row.create_input_text(|input| {
							input.custom_id("content_input")
								.label("Content")
								.min_length(1)
								.required(true)
								.style(InputTextStyle::Paragraph)	
						})
					})
				})
		})
	}).await;

	if modal.is_err() {
		println!("Failed to create modal: {}", modal.as_ref().err().unwrap());
		println!("Modal is {:#?}", modal.as_ref());
	}

	return "".to_string();
}

pub async fn add_tag_handle_modal (interaction: ModalSubmitInteraction, ctx: Context) -> String {
	let name_field = interaction.data.components.find_input("name_input");
	let content_field = interaction.data.components.find_input("content_input");

	let name = &name_field.unwrap().value;
	let content = &content_field.unwrap().value;

	let id_parts: Vec<&str> = interaction.data.custom_id.split("-").collect();

	if id_parts.len() < 3 {
		handle_error!(send_modal_message(ctx, interaction, "Failed to parse modal id", true).await, "Failed to send modal id parse error");
		return "".to_string();
	}

	let tag_type_name = id_parts.get(2);

	let tag_type = match tag_type_name.unwrap() {
		&"lua" => TagType::Lua,
		_ => TagType::Legacy,
	};

	println!("Creating type {:#?}", tag_type);

	if name.is_empty() {
		handle_error!(send_modal_message(ctx, interaction, "Expected name to be provided.", true).await, "Failed to send empty name modal error");
		return "".to_string();
	}

	if content.is_empty() {
		handle_error!(send_modal_message(ctx, interaction, "Expected content to be provided.", true).await, "Failed to send empty content modal error");
		return "".to_string();
	}


	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_ok() {
		return format!("That tag already exists!");
	}

	let result = TagsTable::add_tag(name.clone(), content.to_string(), interaction.user.id.to_string(), Some(tag_type)).await;


	if result.is_ok() {
		handle_error!(send_modal_message(ctx, interaction.clone(), &format!("Added tag {}", name), false).await, "Failed to send tag add success message");
		return "".to_string();
	} else {
		println!("Error adding tag: {:?}", result.err());
		handle_error!(send_modal_message(ctx, interaction.clone(), "Error while adding tag", false).await, "Failed to send tag add error message");

		return "".to_string();
	}
}
