use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::interaction::{
		application_command::ApplicationCommandInteraction,
		message_component::MessageComponentInteraction, modal::ModalSubmitInteraction,
	},
	prelude::Context,
};
use std::future::Future;
use std::pin::Pin;

pub type CommandExecutorFn = fn(
	data: ApplicationCommandInteraction,
	ctx: Context,
) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;

pub type CommandModalHandlerFn = fn(
	data: ModalSubmitInteraction,
	ctx: Context,
) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;
pub type CommandComponentHandlerFn =
	fn(
		data: MessageComponentInteraction,
		ctx: Context,
	) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;

pub type OptionCreatorFn =
	fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
