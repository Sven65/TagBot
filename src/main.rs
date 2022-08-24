
use dotenv::dotenv;
use std::env;
use serenity::{
	Client,
	prelude::{GatewayIntents, EventHandler, Context},
	async_trait,
	model::prelude::{Ready, interaction::{InteractionResponseType, Interaction}
}};

use std::{pin::Pin};

// use futures::{Future, FutureExt};

struct Handler;

mod services;
mod commands;
mod util;


#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Received command interaction: {:#?}", command);
			let index = &commands::framework::COMMAND_INDEX;
			let mut locked_index = index.lock().await;
			let cloned = locked_index.clone();


			let stored_command = cloned.commands.get(command.data.name.as_str());

			let content = match stored_command {
				Some(stored) => {
					let executor = stored.executor;
					let result = executor(command.clone(), ctx.clone()).await;

					result
				},
				None => {
					locked_index.remove_command(command.data.id).await;
					"Invalid command.".to_string()
				}
			};

			if !stored_command.unwrap().sends_message {
				if let Err(why) = command
					.create_interaction_response(&ctx.http, |response| {
						response
							.kind(InteractionResponseType::ChannelMessageWithSource)
							.interaction_response_data(|message| message.content(content))
					})
					.await
					{
						println!("Cannot respond to stupidf command {}", why);
					}
			}
		}
	}


	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected", ready.user.name);

		commands::framework::COMMAND_INDEX.lock().await.set_ctx(ctx.clone());

		commands::init_commands().await;

	}
}

#[tokio::main]
async fn main() {
	dotenv().ok();

	let token = env::var("BOT_TOKEN").expect("Expected bot token to be present in env.");

	let mut client = Client::builder(token, GatewayIntents::empty())
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why)
	}
}