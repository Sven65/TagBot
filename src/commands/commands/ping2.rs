use serenity::{model::prelude::{interaction::application_command::{ApplicationCommandInteraction}}, prelude::Context};

pub async fn ping2(_interaction: ApplicationCommandInteraction, _ctx: Context) -> String {
	return "Ping 2, go away".to_string();
}