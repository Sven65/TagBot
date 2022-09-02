
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::{Regex, Captures};
use serenity::{model::{prelude::{interaction::{application_command::{ApplicationCommandInteraction, CommandData}}, Member, ChannelId, GuildId}, user::User}, prelude::Context};

use crate::{services::rethinkdb::tags::Tag, util::command_options::FindOption, tags::legacy::args::parse_mentions};

use rand::{seq::SliceRandom, Rng};


/// Parses and replaces positional variables, i.e {0}, {1} etc
/// 
/// # Arguments
/// 
/// * `content` - The content to replace in
/// * `data` - Command interaction data to use when parsing
pub fn replace_pos_variables(content: String, data: &CommandData) -> String {
	lazy_static! {
		static ref POS_VAR_REGEX: Regex = Regex::new(r"\{(\d+)\}").unwrap();
	}

	let args_opt = data.find_option("args");

	if args_opt.is_none() {
		return content
	}

	let args = args_opt.unwrap();
	let args = args.value.as_ref().unwrap().as_str().unwrap();

	let args: Vec<&str> = args.split(" ").collect();

	let res = POS_VAR_REGEX.replace_all(&content, |caps: &Captures| {

		let pos = caps.get(1);

		if pos.is_none() {
			return caps.get(0).unwrap().as_str().to_string();
		}

		let pos: i32 = pos.unwrap().as_str().to_string().parse::<i32>().unwrap();
		let pos = pos as usize;
 
		if pos >= args.len() {
			return caps.get(0).unwrap().as_str().to_string();
		}

		let pos_arg = args[pos];

		return pos_arg.to_string();
	});

	return res.to_string();
}

/// Parses, executes and replaces chooser tags in the content.
/// 
/// {choose(1|2|3)} would be replaced by 1, 2 or 3.
/// 
/// # Arguments
/// 
/// * `content` - The content to replace
pub fn replace_choosers(content: String) -> String {
	lazy_static! {
		static ref CHOOSE_VAR_REGEX: Regex = Regex::new(r"\{choose\((.*?)\)\}").unwrap();
	}

	let res = CHOOSE_VAR_REGEX.replace_all(&content, |caps: &Captures| {
		let options: Vec<&str> = caps.get(1).unwrap().as_str().split("|").collect();

		let option = options.choose(&mut rand::thread_rng());

		return option.unwrap().to_string();
	});

	return res.to_string();
}


/// Parses, executes and replaces rint tags in the content.
/// 
/// {rint(1,6)} would be replaced by a number between 1 and 6
/// 
/// # Arguments
/// 
/// * `content` - The content to replace
pub fn replace_rint(content: String) -> String {
	lazy_static! {
		static ref RINT_REGEX: Regex = Regex::new(r"\{rint\((\d+),\s*(\d+)\)\}").unwrap();
	}

	let res = RINT_REGEX.replace_all(&content, |caps: &Captures| {

		let min = caps.get(1).unwrap();
		let max = caps.get(2).unwrap();

		let min: i32 = min.as_str().parse::<i32>().unwrap();
		let max: i32 = max.as_str().parse::<i32>().unwrap();

		let mut rng = rand::thread_rng();

		let val = rng.gen_range(min..max);

		return val.to_string();
	});

	return res.to_string();
}

/// Formats the content with datetime formatter tags, i.e %H etc
/// 
/// # Arguments
/// 
/// * `content` - The content to replace
pub fn replace_dates(content: String) -> String {
	let now: DateTime<Utc> = Utc::now();

	return now.format(&content).to_string();

}

/// Replaces sender tags with the appropriate data of the passed sender
/// 
/// # Arguments
/// 
/// * `content` - The content to replace
/// * `sender` - The user to use for data
pub fn replace_sender_variables(content: String, sender: &User) -> String {
	return content
		.replacen("{sname}", &sender.name, 1)
		.replacen("{sid}", &sender.id.to_string(), 1)
		.replacen("{sdiscrim}", &sender.discriminator.to_string(), 1)
		.replacen("{sbot}", &sender.bot.to_string(), 1)
		.replacen("{sender}", &format!("<@{}>", sender.id), 1);
}


/// Replaces sender member tags with the appropriate data of the passed member
/// 
/// # Arguments
/// 
/// * `content` - The content to replace
/// * `member` - The user to use for data
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


/// Replaces channel tags with the appropriate data of the passed channel id
/// 
/// # Arguments
/// 
/// * `ctx` - The serenity content to use for fetching the channel
/// * `content` - The content to replace
/// * `channel_id` - The id of the channel to use for data
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

/// Replaces server tags with the appropriate data of the passed server id
/// 
/// # Arguments
/// 
/// * `ctx` - The serenity content to use for fetching the server
/// * `content` - The content to replace
/// * `guild_id` - The id of the guild to use for data
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

/// Replaces server owner tags with the appropriate data of the owner of the passed guild id
/// 
/// # Arguments
/// 
/// * `ctx` - The serenity content to use for fetching the guild owner
/// * `content` - The content to replace
/// * `guild_id` - The id of the server to use for data
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


/// Replaces channel tags with the appropriate data of the passed mention
/// 
/// # Arguments
/// 
/// * `ctx` - The serenity content to use for fetching the mentioned member
/// * `content` - The content to replace
/// * `data` - The command interaction data to use when parsing the mention
/// * `guild_id` - The id of the guild to use for fetching the mentioned member
async fn replace_mention_variables (ctx: &Context, content: String, data: &CommandData, guild_id: Option<GuildId>) -> String {
	let args_opt = data.find_option("args");

	if args_opt.is_none() {
		return content
	}

	let mut ret_content = content.clone();

	let args = args_opt.unwrap();
	let args = args.value.as_ref().unwrap().as_str().unwrap();
	

	let mentions = parse_mentions(args);

	// Only use first mention

	if mentions.len() == 0 {
		return ret_content
	}

	let mentioned_user = mentions[0].to_user(&ctx.http).await;

	if mentioned_user.is_err() {
		return ret_content
	}

	let mentioned_user = mentioned_user.unwrap();

	ret_content = ret_content.replacen("{mentionname}", &mentioned_user.name, 1)
		.replacen("{mentionid}", mentioned_user.id.to_string().as_str(), 1)
		.replacen("{mentiondiscrim}", mentioned_user.discriminator.to_string().as_str(), 1)
		.replacen("{mentionbot}", &mentioned_user.bot.to_string(),  1)
		.replacen("{mention}", format!("<@{}>", &mentioned_user.id.to_string()).as_str(), 1);

	if guild_id.is_some() {
		// Not a DM

		let guild = guild_id.unwrap();

		let guild_member = guild.member(&ctx.http, mentions[0]).await;

		if guild_member.is_ok() {
			let guild_member = guild_member.unwrap();

			let mut joined_at_string = "Unknown".to_string();

			if guild_member.joined_at.is_some() {
				let joined_at_ts = guild_member.joined_at.unwrap().unix_timestamp();
				joined_at_string = format!("<t:{}:F>", &joined_at_ts);
			}


			ret_content = ret_content.replacen("{mentionjoined}", joined_at_string.as_str(), 1)
				.replacen("{mentionnick}", guild_member.nick.unwrap_or("Unknown".to_string()).as_str(), 1);
		}
	}

	return ret_content;
}


pub async fn execute_tag(tag: Tag, interaction: ApplicationCommandInteraction, ctx: Context) -> String {
	
	let mut content = tag.content;

	content = replace_pos_variables(content, &interaction.data);
	content = replace_dates(content);

	content = replace_sender_variables(content, &interaction.user);
	content = replace_sender_member_variables(content, interaction.member);
	content = replace_channel_variables(&ctx, content, interaction.channel_id).await;
	content = replace_server_variables(&ctx, content, interaction.guild_id).await;
	content = replace_server_owner_variables(&ctx, content, interaction.guild_id).await;
	content = replace_mention_variables(&ctx, content, &interaction.data, interaction.guild_id).await;

	content = replace_choosers(content);
	content = replace_rint(content);

	

	return content
}

