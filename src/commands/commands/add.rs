use std::io::Error;

use reql::{cmd::connect, r};
use serenity::{model::prelude::{interaction::application_command::{CommandData, CommandDataOptionValue}, command::CommandOptionType}, builder::CreateApplicationCommand};
use futures::TryStreamExt;

use crate::{util::command_options::*, services::rethinkdb::{RDB, Tag}};

async fn add_tag(name: String, content: String) -> Result<String, reql::Error> {
	let mut locked = RDB.lock().unwrap();

	let connection = locked.getConnection().await;

	if connection.is_none() {
		return Ok("Failed to create tag: Failed to get DB Connection.".to_string());
	}

	let connection = connection.unwrap();

	let tag = Tag {
		id: name.as_str(),
		content: content.as_str(),
		owner: "141610251299454976",
	};

	let mut query = r.table("tags").insert(tag).run(connection);

	if let Some(result) = query.try_next().await? {
		println!("Result {}", result);
		return Ok("Done".to_string());
    }

	return Ok("OK".to_string());
}

pub async fn add(data: CommandData) -> String {
	let name = data.find_option("name")
		.expect("Expected name option")
		.resolved
		.as_ref()
		.expect("Expected name value");

	let contents = data.find_option("content")
		.expect("Expected content option")
		.resolved
		.as_ref()
		.expect("Expected content value");
		

	let name: String = match name {
		CommandDataOptionValue::String(option) => {option.to_string()}
		&_ => { "Invalid name".to_string() }
	};

	let contents = match contents {
		CommandDataOptionValue::String(option) => {option.to_string()}
		&_ => { "Invalid contents".to_string() }
	};


	add_tag(name, contents);

	// if result.is_ok() {
	// 	return format!("Creating tag {} with contents {}", name, contents);
    // }

	// return "Failed to create tag".to_string();

	return "fuck".to_string();
}


pub fn add_options_creator(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.create_option(|option| {
		option.name("name")
		.kind(CommandOptionType::String)
		.description("The name of the tag")
		.required(true)
	});

	let data = command.create_option(|option| {
		option.name("content")
		.kind(CommandOptionType::String)
		.description("The contents of the tag")
		.required(true)
	});

	return data;
}
