
use dotenv::dotenv;
use std::env;
use serenity::{
	Client,
	prelude::{GatewayIntents, EventHandler, Context},
	async_trait,
	model::prelude::{Ready, interaction::{InteractionResponseType, Interaction}
}};


struct Handler;

mod services;
mod commands;
mod util;
mod tags;


#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			// println!("Received command interaction: {:#?}", command);
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
							.interaction_response_data(|message| message.content(content).ephemeral(false))
					})
					.await
					{
						println!("Cannot respond to stupidf command {}", why);
					}
			}
		} else if let Interaction::ModalSubmit(modal) = interaction {
			if modal.data.custom_id.is_empty() {
				panic!("Received modal has no custom id.");
			}

			let split = modal.data.custom_id.split("-");

			let split = split.collect::<Vec<&str>>();

			if split[0].is_empty() {
				panic!("No command found in modal custom id.");
			}


			let index = &commands::framework::COMMAND_INDEX;
			let locked_index = index.lock().await;
			let cloned = locked_index.clone();


			let stored_command = cloned.commands.get(split[0]);

			if stored_command.is_none() {
				panic!("Can't find command for {}", split[0])
			}

			let stored_command = stored_command.unwrap();

			if stored_command.modal_handler.is_some() {
				let handler = stored_command.modal_handler.unwrap();

				handler(modal, ctx).await;
			} else {
				panic!("No modal handler found for {}", split[0]);
			}
		} else if let Interaction::MessageComponent(component) = interaction {
			println!("Received component interaction: {:#?}", component);

			if component.data.custom_id.is_empty() {
				panic!("Received modal has no custom id.");
			}

			let split = component.data.custom_id.split("-");

			let split = split.collect::<Vec<&str>>();

			if split[0].is_empty() {
				panic!("No command found in component custom id.");
			}

			let index = &commands::framework::COMMAND_INDEX;
			let locked_index = index.lock().await;
			let cloned = locked_index.clone();


			let stored_command = cloned.commands.get(split[0]);

			if stored_command.is_none() {
				panic!("Can't find command for {}", split[0])
			}

			let stored_command = stored_command.unwrap();

			if stored_command.modal_handler.is_some() {
				let handler = stored_command.component_handler.unwrap();

				handler(component, ctx).await;
			} else {
				panic!("No component handler found for {}", split[0]);
			}

		}
	}


	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected", ready.user.name);

		commands::framework::COMMAND_INDEX.lock().await.set_ctx(ctx.clone());

		commands::init_commands().await;

		tags::lua::lua_modules::registry::init::init_modules().await;
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