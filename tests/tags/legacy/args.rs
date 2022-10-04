use tagbot::tags::legacy::args::parse_mentions;
use test_case::test_case;

#[test_case("<@!178790340289888264>", 1 ; "When one mention is present, one should be parsed")]
#[test_case("<@!178790343289888264> <@!178790333289888264>", 2 ; "When two mentions are present, two should be parsed")]
#[test_case("no mentions", 0 ; "When no mentions are present, none should be parsed")]
fn parse_mentions_test(args: &str, mentions_count: usize) {
	let mentions = parse_mentions(args);

	assert_eq!(mentions.len(), mentions_count);
}
