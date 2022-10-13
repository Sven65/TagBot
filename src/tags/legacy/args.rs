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
				let u64_id = id_value.as_str().parse::<u64>();

				if let Ok(u64_id_value) = u64_id {
					let user_id = UserId(u64_id_value);

					mention_ids.push(user_id);
				}
			}
		}
	}

	mention_ids
}
