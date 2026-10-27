use lazy_static::lazy_static;
use regex::Regex;
use serenity::model::prelude::UserId;

pub fn parse_mentions(args: &str) -> Vec<UserId> {
	lazy_static! {
		static ref MENTION_REGEX: Regex = Regex::new(r"<@!?(\d+)>").unwrap();
	}

	let matches = MENTION_REGEX.find_iter(args);

	let mut mention_ids: Vec<UserId> = Vec::new();

	for m in matches {
		for cap in MENTION_REGEX.captures_iter(m.as_str()) {
			let id = cap.get(1);

			if let Some(id_value) = id {
				let user_id = id_value.as_str().parse::<UserId>();

				if let Ok(user_id_value) = user_id {
					mention_ids.push(user_id_value);
				}
			}
		}
	}

	mention_ids
}
