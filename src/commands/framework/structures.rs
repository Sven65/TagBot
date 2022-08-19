

pub trait Command {
	fn name() -> String;
	fn description() -> String;

	fn execute(&self);
}