use cat_loggr::log_fatal;
use serenity::{
	builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup},
	model::prelude::{CommandDataOptionValue, CommandInteraction, CommandOptionType},
	prelude::Context,
};

use crate::{
	handle_error,
	services::rethinkdb::tags::{TagType, TagsTable},
	tags::{legacy::executor::execute_tag, lua::executor::execute_tag as execute_lua_tag},
	util::{command_options::*, message::send_app_interaction_message},
};

pub async fn tag(interaction: CommandInteraction, ctx: Context) -> String {
	let name = &interaction
		.data
		.find_option("name")
		.expect("Expected name option")
		.value;

	let name: String = match name {
		CommandDataOptionValue::String(option) => option.to_string(),
		&_ => "Invalid name".to_string(),
	};

	let gotten_tag = TagsTable::get_tag(name.clone()).await;

	if gotten_tag.is_err() {
		cat_loggr::log_fatal!("{}: {:?}", "Failed to get tag (1):", gotten_tag.err());
		handle_error!(
			send_app_interaction_message(ctx, interaction, "That tag doesn't exist", false).await,
			"Failed to send non-existant tag message"
		);
		"".to_string()
	} else {
		handle_error!(
			interaction.defer(&ctx.http).await,
			"Failed to defer tag execution"
		);
		let tag = gotten_tag.unwrap();

		// Execute tag
		let data = match tag.tag_type {
			Some(TagType::Legacy) => execute_tag(tag, interaction.clone(), ctx.clone()).await,
			Some(TagType::Lua) => execute_lua_tag(tag, interaction.clone(), ctx.clone()).await,
			_ => execute_tag(tag, interaction.clone(), ctx.clone()).await,
		};

		let data = match data {
			Ok(res) => res,
			Err(res) => res.to_string(),
		};

		if data.trim() == "nil" {
			return "".to_string();
		}

		let res = interaction
			.create_followup(
				&ctx.http,
				CreateInteractionResponseFollowup::new().content(data),
			)
			.await;

		if res.is_err() {
			log_fatal!(
				"Failed to send tag content follow up message: {:#?}",
				res.err()
			);
		}

		"".to_string()
	}
}

pub fn tag_options_creator(command: CreateCommand) -> CreateCommand {
	command
		.add_option(
			CreateCommandOption::new(CommandOptionType::String, "name", "The name of the tag")
				.required(true),
		)
		.add_option(CreateCommandOption::new(
			CommandOptionType::String,
			"args",
			"Arguments for the tag",
		))
}
