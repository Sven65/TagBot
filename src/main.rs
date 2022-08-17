
use dotenv::dotenv;
use std::env;
use serenity::{
	Client,
	prelude::{GatewayIntents, EventHandler, Context},
	async_trait,
	model::prelude::{Ready, command::Command, interaction::{InteractionResponseType, Interaction}
}};

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub use tagbot::commands::framework::commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Received command interaction: {:#?}", command);

			let content = match command.data.name.as_str() {
				"ping" => "shut up, go away".to_string(),
				_ => "not fucking here dumbo".to_string(),
			};

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


	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected", ready.user.name);
	
		// let guildId = GuildId(
		//     env::var("GUILD_ID")
		//     .expect("Expected GUILD_ID in env.")
		//     .parse()
		//     .expect("GUILD_ID must be integer"),
		// );
		
		let guild_command = Command::create_global_application_command(&ctx.http, |command| {
			command.name("ping").description("xd")
		})
		.await;

		println!("I created the following global slash command: {:#?}", guild_command);
	}
}


#[tokio::main]
async fn main() {
	let subscriber = FmtSubscriber::builder()
	// all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
	// will be written to stdout.
	.with_max_level(Level::TRACE)
	// completes the builder.
	.finish();

tracing::subscriber::set_global_default(subscriber)
	.expect("setting default subscriber failed");

	dotenv().ok();

	let token = env::var("BOT_TOKEN").expect("Expected bot token to be present in env.");

	let mut client = Client::builder(token, GatewayIntents::empty())
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	if let Err(why) = client.start().await {
		println!("Cleitn error: {:?}", why)
	}
}