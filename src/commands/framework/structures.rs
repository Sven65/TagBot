use serenity::{builder::CreateApplicationCommand};



pub trait Command {
	fn name() -> String;
	fn description() -> String;

	fn execute(&self);
}

pub type OptionCreatorFn = fn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;