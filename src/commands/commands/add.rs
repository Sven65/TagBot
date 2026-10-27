use cat_loggr::{log_debug, log_fatal};
use serenity::{
	builder::{
		CreateActionRow, CreateCommand, CreateCommandOption, CreateInputText,
		CreateInteractionResponse, CreateModal,
	},
	model::prelude::{
		CommandDataOptionValue, CommandInteraction, CommandOptionType, InputTextStyle,
		ModalInteraction,
	},
	prelude::Context,
};

use crate::{
	handle_error,
	services::rethinkdb::tags::{TagType, TagsTable},
	util::{
		command_options::FindOption,
		input_field::FindInput,
		message::{send_app_interaction_message, send_modal_message},
	},
};

pub async fn add(interaction: CommandInteraction, ctx: Context) -> String {
	let tag_type = &interaction
		.data
		.find_option("type")
		.expect("Expected tag type option")
		.value;

	let tag_type: String = match tag_type {
		CommandDataOptionValue::String(option) => option.to_string(),
		&_ => "Invalid tag type".to_string(),
	};

	let tag_type = match tag_type.to_lowercase().as_str() {
		"regular" => TagType::Legacy,
		"lua" => TagType::Lua,
		&_ => TagType::Invalid,
	};

	if tag_type == TagType::Invalid {
		handle_error!(
			send_app_interaction_message(
				ctx,
				interaction,
				"Expected tag type to be one of lua or regular.",
				true
			)
			.await,
			"Failed to send invalid tag type error"
		);
		return "".to_string();
	}

	// Send modal itself

	let name_field = interaction.data.find_option("name");
	let mut tag_name: Option<String> = None;

	if name_field.is_some() {
		let name = &name_field.unwrap().value;

		let tag_name_value = match name {
			CommandDataOptionValue::String(option) => option.to_string(),
			_ => "Invalid tag name type".to_string(),
		};

		tag_name = Some(tag_name_value);
	}

	let mut name_input = CreateInputText::new(InputTextStyle::Short, "Name", "name_input")
		.min_length(1)
		.required(true);

	if let Some(tname) = tag_name {
		name_input = name_input.value(tname);
	}

	let content_input = CreateInputText::new(InputTextStyle::Paragraph, "Content", "content_input")
		.min_length(1)
		.required(true);

	let modal = CreateModal::new(
		format!("add-{}-{}", interaction.user.id, tag_type),
		"Create a new tag",
	)
	.components(vec![
		CreateActionRow::InputText(name_input),
		CreateActionRow::InputText(content_input),
	]);

	let modal = interaction
		.create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
		.await;

	if modal.is_err() {
		log_fatal!("Failed to create modal: {}", modal.as_ref().err().unwrap());
		log_debug!("Modal is {:#?}", modal.as_ref());
	}

	"".to_string()
}

pub async fn add_tag_handle_modal(interaction: ModalInteraction, ctx: Context) -> String {
	let name_field = interaction.data.components.find_input("name_input");
	let content_field = interaction.data.components.find_input("content_input");

	let name = name_field.unwrap().value.as_ref().unwrap();
	let content = content_field.unwrap().value.as_ref().unwrap();

	let id_parts: Vec<&str> = interaction.data.custom_id.split('-').collect();

	if id_parts.len() < 3 {
		handle_error!(
			send_modal_message(ctx, interaction, "Failed to parse modal id", true).await,
			"Failed to send modal id parse error"
		);
		return "".to_string();
	}

	let tag_type_name = id_parts.get(2);

	let tag_type = match tag_type_name.unwrap() {
		&"lua" => TagType::Lua,
		_ => TagType::Legacy,
	};

	log_debug!("Creating type {:#?}", tag_type);

	if name.is_empty() {
		handle_error!(
			send_modal_message(ctx, interaction, "Expected name to be provided.", true).await,
			"Failed to send empty name modal error"
		);
		return "".to_string();
	}

	if content.is_empty() {
		handle_error!(
			send_modal_message(ctx, interaction, "Expected content to be provided.", true).await,
			"Failed to send empty content modal error"
		);
		return "".to_string();
	}

	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_ok() {
		handle_error!(
			send_modal_message(ctx, interaction, "That tag already exists.", true).await,
			"Failed to send tag exists content modal error"
		);

		return "That tag already exists!".to_string();
	}

	let result = TagsTable::add_tag(
		name.clone(),
		content.to_string(),
		interaction.user.id.to_string(),
		Some(tag_type),
	)
	.await;

	if result.is_ok() {
		handle_error!(
			send_modal_message(
				ctx,
				interaction.clone(),
				&format!("Added tag {}", name),
				false
			)
			.await,
			"Failed to send tag add success message"
		);
		"".to_string()
	} else {
		log_fatal!("Error adding tag: {:?}", result.err());
		handle_error!(
			send_modal_message(ctx, interaction.clone(), "Error while adding tag", false).await,
			"Failed to send tag add error message"
		);

		"".to_string()
	}
}

pub fn add_options_creator(command: CreateCommand) -> CreateCommand {
	command
		.add_option(
			CreateCommandOption::new(CommandOptionType::String, "type", "The type of the tag")
				.add_string_choice("Regular", "regular")
				.add_string_choice("Lua", "lua")
				.required(true),
		)
		.add_option(CreateCommandOption::new(
			CommandOptionType::String,
			"name",
			"The name of the tag",
		))
}
