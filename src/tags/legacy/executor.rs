
use std::any::Any;

use serenity::{model::{prelude::{interaction::{application_command::ApplicationCommandInteraction, self}, Member, ChannelId, GuildId}, user::User}, prelude::Context};

use crate::{services::rethinkdb::tags::Tag};


pub fn replace_sender_variables(content: String, sender: &User) -> String {
	return content
		.replacen("{sname}", &sender.name, 1)
		.replacen("{sid}", &sender.id.to_string(), 1)
		.replacen("{sdiscrim}", &sender.discriminator.to_string(), 1)
		.replacen("{sbot}", &sender.bot.to_string(), 1)
		.replacen("{sender}", &format!("<@{}>", sender.id), 1);
}

pub fn replace_sender_member_variables(content: String, member: Option<Member>) -> String {
	if member.is_none() {
		return content;
	}

	let member = member.unwrap();

	let mut joined_at_string = "Unknown".to_string();

	if member.joined_at.is_some() {
		let joined_at_ts = member.joined_at.unwrap().unix_timestamp();
		joined_at_string = format!("<t:{}:F>", &joined_at_ts);
	}

	return content
		.replacen("{sjoined}", joined_at_string.as_str(), 1)
		.replacen("{snick}", &member.nick.as_ref().unwrap_or(&member.user.name.clone()), 1)
}


async fn replace_channel_variables(ctx: &Context, content: String, channel_id: ChannelId) -> String {
	let channel = channel_id.to_channel(&ctx.http).await;

	if channel.is_err() {
		return content;
	}

	let channel = channel.unwrap();

	let guild_channel = channel.guild();

	if guild_channel.is_none() {
		return content;
	}

	let guild_channel = guild_channel.unwrap();

	return content
		.replacen("{chanid}", format!("{}", guild_channel.id.as_u64()).as_str(), 1)
		.replacen("{chantype}", guild_channel.kind.name(), 1)
		.replacen("{channame}", guild_channel.name(), 1)
		.replacen("{chantopic}", guild_channel.topic.unwrap_or("Unknown".to_string()).as_str(), 1)

}

async fn replace_server_variables(ctx: &Context, content: String, guild_id: Option<GuildId>) -> String {
	if guild_id.is_none() {
		return content;
	}

	let guild = guild_id.unwrap().to_partial_guild(&ctx.http).await;

	if guild.is_err() {
		return content;
	}

	let guild = guild.unwrap();

	let mut member_count_str = "Unknown".to_string();


	if guild.approximate_member_count.is_some() {
		member_count_str = format!("{}", guild.approximate_member_count.unwrap());
	} else {
		println!("Member count is None.")
		// TODO: Try making this fetch members
	}

	let mut channel_count_str = "Unknown".to_string();


	let channels = guild.channels(&ctx.http).await;

	if channels.is_ok() {
		channel_count_str = format!("{}", channels.unwrap().len());
	} else {
		println!("Failed to get channel count: {:?}", channels.err())
	}


	return content
		.replacen("{servername}", &guild.name, 1)
		.replacen("{serverid}", format!("{}", guild.id.as_u64()).as_str(), 1)
		.replacen("{servermembs}", member_count_str.as_str(), 1)
		.replacen("{serverchans}", channel_count_str.as_str(), 1)
		.replacen("{serverdefchan}", format!("<#{}>", guild.id.as_u64()).as_str(), 1)
		.replacen("{serververification}", format!("{:?}", &guild.verification_level).as_str(), 1);
}

async fn replace_server_owner_variables (ctx: &Context, content: String, guild_id: Option<GuildId>) -> String {
	if guild_id.is_none() {
		return content;
	}

	let guild = guild_id.unwrap().to_partial_guild(&ctx.http).await;

	if guild.is_err() {
		return content;
	}

	let guild = guild.unwrap();

	let owner = guild.owner_id.to_user(&ctx.http).await;

	if owner.is_err() {
		println!("Failed to get owner of guild: {:?}", owner.err());
		return content
	}

	let owner = owner.unwrap();

	let owner_member = guild.member(&ctx.http, owner.id).await;

	if owner_member.is_err() {
		println!("Failed to get owner member of guild: {:?}", owner_member.err());
		return content
	}

	let owner_member = owner_member.unwrap();


	let mut joined_at_string = "Unknown".to_string();

	if owner_member.joined_at.is_some() {
		let joined_at_ts = owner_member.joined_at.unwrap().unix_timestamp();
		joined_at_string = format!("<t:{}:F>", &joined_at_ts);
	}

	return content
		.replacen("{serverowner}", &owner.name, 1)
		.replacen("{serverownername}", &owner.name, 1)
		.replacen("{serverownernick}", &owner_member.nick.unwrap_or("Unknown".to_string()).as_str(), 1)
		.replacen("{serverownerid}", format!("{}", owner.id.as_u64()).as_str(), 1)
		.replacen("{serverownerjoined}", &joined_at_string, 1);

}

pub fn ee_tag(tag: Tag, interaction: ApplicationCommandInteraction) -> String {
	return tag.content.replacen("", "", 1);
}




pub async fn execute_tag(tag: Tag, interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	
	let mut content = tag.content;

	content = replace_sender_variables(content, &interaction.user);
	content = replace_sender_member_variables(content, interaction.member);
	content = replace_channel_variables(&ctx, content, interaction.channel_id).await;
	content = replace_server_variables(&ctx, content, interaction.guild_id).await;
	content = replace_server_owner_variables(&ctx, content, interaction.guild_id).await;
	

	return content

    // let user = User {
    //     id: "123".to_string(),
    //     name: "User name".to_string(),
    //     discrim: "#1934".to_string(),
    // };
    
    // let content: String = "id: {id}, name: {sname}, discrim: {discrim}".to_string();

	// let formatted = replace_sender_variables(content, user);
    
    // println!("{}", formatted);
}

