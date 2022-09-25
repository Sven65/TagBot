use lazy_static::lazy_static;
use regex::Regex;
use serenity::model::prelude::UserId;

pub fn parse_mentions (args: &str) -> Vec<UserId> {
	lazy_static! {
		static ref MENTION_REGEX: Regex = Regex::new(r"<@!?(\d+)>").unwrap();
	}

	let matches = MENTION_REGEX.find_iter(args);

	let mut mention_ids: Vec<UserId> = Vec::new();

	for m in matches {
		for cap in MENTION_REGEX.captures_iter(m.as_str()) {
			let id = cap.get(1);

			if id.is_some() {
				let u64_id = u64::from_str_radix(id.unwrap().as_str(), 10);

				if u64_id.is_ok() {
					let user_id = UserId(u64_id.unwrap());

					mention_ids.push(user_id);
				}
			}
		}
	}

	return mention_ids;
}
