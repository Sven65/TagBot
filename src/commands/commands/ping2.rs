use serenity::model::prelude::{interaction::application_command::{ApplicationCommandInteraction}};

pub async fn ping2(_interaction: ApplicationCommandInteraction) -> String {
	return "Ping 2, go away".to_string();
}