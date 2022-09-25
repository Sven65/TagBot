use serenity::model::{user::User};
use tagbot::tags::legacy::executor::replace_sender_variables;
use test_case::test_case;



fn get_test_user() -> User {
	let mut test_user = User::default();

	test_user.id = 141610251299454976.into();
	test_user.bot = false;
	test_user.discriminator = 7196;
	test_user.name = "Mackan".to_string();


	return test_user;
}

#[test_case("{sname}", "Mackan"; "{sname} should be replaced by correct name")]
#[test_case("{sid}", "141610251299454976" ; "{sid} should be replaced correctly")]
#[test_case("{sdiscrim}", "7196" ; "{sdiscrim} should be replaced correctly")]
#[test_case("{sbot}", "false" ; "{sbot} should be replaced correctly")]
#[test_case("{sender}", "<@141610251299454976>" ; "{sender} should be replaced correctly")]
fn replace_sender_variables_test(args: &str, expected: &str) {
	let test_user = get_test_user();
	let replaced = replace_sender_variables(args.to_string(), &test_user);

	assert_eq!(replaced, expected.to_string());
}