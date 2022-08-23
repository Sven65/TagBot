use std::pin::Pin;
use std::future::Future;
use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::application_command::CommandData};

// fn wrap<F>(_f: F) -> CommandExecution
// where
//     F: 'static + for<'a> CommandExecutionAsyncFn<&'a CommandExecutor>,
// {
//     assert_eq!(std::mem::size_of::<F>(), 0, "expected a fn item");
//     move |executor, command| {
//         // SAFETY: `F` is a ZST (checked above), any (aligned!) pointer, even crafted
//         // out of the thin air, is valid for it.
//         let f: &F = unsafe { std::ptr::NonNull::dangling().as_ref() };
//         Box::pin(f(executor, command))
//     }
// }

pub type CommandExecutorFn = fn(data: CommandData) -> Pin<Box<dyn Future<Output = std::string::String> + Send>>;


pub trait Command {
	fn name() -> String;
	fn description() -> String;

	fn execute(&self);
}

pub type OptionCreatorFn = fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;