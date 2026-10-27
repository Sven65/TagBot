use serenity::{model::prelude::CommandInteraction, prelude::Context};

pub async fn ping2(_interaction: CommandInteraction, _ctx: Context) -> String {
	"Ping 2, go away".to_string()
}
