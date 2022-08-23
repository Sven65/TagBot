use serenity::model::prelude::{interaction::application_command::CommandData};

pub async fn ping2(_data: CommandData) -> String {
	return "Ping 2, go away".to_string();
}