use std::pin::Pin;
use std::future::Future;
use serenity::{builder::CreateApplicationCommand, model::prelude::{interaction::{application_command::{ApplicationCommandInteraction}, modal::ModalSubmitInteraction, message_component::MessageComponentInteraction}}, prelude::Context};

pub type CommandExecutorFn = fn(data: ApplicationCommandInteraction, ctx: Context) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;

pub type CommandModalHandlerFn = fn(data: ModalSubmitInteraction, ctx: Context) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;
pub type CommandComponentHandlerFn = fn(data: MessageComponentInteraction, ctx: Context) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;


pub trait Command {
	fn name() -> String;
	fn description() -> String;

	fn execute(&self);
}

pub type OptionCreatorFn = fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;