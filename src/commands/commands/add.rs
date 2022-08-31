use serenity::{model::prelude::{interaction::{application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, InteractionResponseType, modal::ModalSubmitInteraction}, component::InputTextStyle, command::CommandOptionType}, prelude::Context, builder::CreateApplicationCommand};

use crate::{util::{input_field::FindInput, message::{send_modal_message, send_app_interaction_message}, command_options::FindOption}, services::rethinkdb::{tags::{TagsTable, TagType}}, handle_error};

pub async fn add(interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	
	let tag_type = interaction.data.find_option("type")
		.expect("Expected tag type option")
		.resolved
		.as_ref()
		.expect("Expected tag type value");

	let tag_type: String = match tag_type {
		CommandDataOptionValue::String(option) => {option.to_string()}
		&_ => { "Invalid tag type".to_string() }
	};

	let tag_type = match tag_type.to_lowercase().as_str() {
		"regular" => TagType::Legacy,
		"lua" => TagType::Lua,
		&_ => TagType::Invalid,
	};

	if tag_type == TagType::Invalid {
		handle_error!(send_app_interaction_message(ctx, interaction, "Expected tag type to be one of lua or regular.", true).await, "Failed to send invalid tag type error");
		return "".to_string();
	}

	// Send modal itself

	let name_field = interaction.data.find_option("name");
	let mut tag_name: Option<String> = None;

	if name_field.is_some() {
		let name = name_field.unwrap().resolved.as_ref().unwrap();

		let tag_name_value = match name {
			CommandDataOptionValue::String(option) => {option.to_string()}
			_ => { "Invalid tag name type".to_string() }
		};

		tag_name = Some(tag_name_value);
	}

	let modal = interaction.create_interaction_response(&ctx.http, |response| {
		response.kind(InteractionResponseType::Modal)
		.interaction_response_data(|modal| {
			modal.custom_id(format!("add-{}-{}", interaction.user.id, tag_type))
				.title("Create a new tag")
				.components(|comp| {
					

					comp.create_action_row(|row| {
						row.create_input_text(|input| {
							input.custom_id("name_input")
								.label("Name")
								.min_length(1)
								.required(true)
								.style(InputTextStyle::Short);

							if tag_name.is_some() {
								input.value(tag_name.unwrap());
							}

							return input;
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
		handle_error!(send_modal_message(ctx, interaction, "That tag already exists.", true).await, "Failed to send tag exists content modal error");

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


pub fn add_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.create_option(|option|{
		option.name("type")
			.kind(CommandOptionType::String)
			.description("The type of the tag")
			.add_string_choice("Regular", "regular")
			.add_string_choice("Lua", "lua")
			.required(true)
	});

	let data = command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
	});

	return data;
}
