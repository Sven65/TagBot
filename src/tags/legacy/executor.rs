
use serenity::{model::{prelude::{interaction::{application_command::ApplicationCommandInteraction, self}, Member, ChannelId}, user::User}, prelude::Context};

use crate::services::rethinkdb::tags::Tag;


pub fn replace_sender_variables(content: String, sender: &User) -> String {
	return content
		.replacen("{sname}", &sender.name, 1)
		.replacen("{sid}", &sender.id.to_string(), 1)
		.replacen("{sdiscrim}", &sender.discriminator.to_string(), 1)
		.replacen("{sbot}", &sender.bot.to_string(), 1)
		.replacen("{sender}", &format!("<@{}>", sender.id), 1);
}

pub fn replace_sender_member_variables(content: String, member: &Member) -> String {
	let mut joined_at_string = "Unknown".to_string();

	if member.joined_at.is_some() {
		let joined_at_ts = member.joined_at.unwrap().unix_timestamp();
		joined_at_string = format!("<t:{}:F>", &joined_at_ts);
	}

	return content
		.replacen("{sjoined}", joined_at_string.as_str(), 1)
		.replacen("{snick}", &member.nick.as_ref().unwrap_or(&member.user.name.clone()), 1)
}

// fn replace_message_variables (content: String, interaction: ApplicationCommandInteraction) -> String {
// 	return content
// 		.replacen("{mtime}", interaction.data., 1)
// 		.replacen("{md}", &member.nick.as_ref().unwrap_or(&member.user.name.clone()), 1)
// }

async fn replace_channel_variables(ctx: Context, content: String, channel_id: ChannelId) -> String {
	let channel = channel_id.to_channel(ctx.http).await;

	if channel.is_err() return "".to_string();

	return "".to_string();
}

pub fn ee_tag(tag: Tag, interaction: ApplicationCommandInteraction) -> String {
	return tag.content.replacen("", "", 1);
}




pub fn execute_tag() {
	
    // let user = User {
    //     id: "123".to_string(),
    //     name: "User name".to_string(),
    //     discrim: "#1934".to_string(),
    // };
    
    // let content: String = "id: {id}, name: {sname}, discrim: {discrim}".to_string();

	// let formatted = replace_sender_variables(content, user);
    
    // println!("{}", formatted);
}

