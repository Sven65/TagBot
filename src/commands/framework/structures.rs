use serenity::{
	builder::CreateCommand,
	model::prelude::{CommandInteraction, ComponentInteraction, ModalInteraction},
	prelude::Context,
};
use std::future::Future;
use std::pin::Pin;

pub type CommandExecutorFn = fn(
	data: CommandInteraction,
	ctx: Context,
) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;

pub type CommandModalHandlerFn = fn(
	data: ModalInteraction,
	ctx: Context,
) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;
pub type CommandComponentHandlerFn =
	fn(
		data: ComponentInteraction,
		ctx: Context,
	) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;

pub type OptionCreatorFn = fn(command: CreateCommand) -> CreateCommand;
