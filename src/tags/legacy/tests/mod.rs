
#[cfg(test)]
mod tests {
    use serenity::model::{user::User};

    use crate::tags::legacy::executor::replace_sender_variables;



	fn get_test_user() -> User {
		let mut test_user = User::default();

		test_user.id = 141610251299454976.into();
		test_user.bot = false;
		test_user.discriminator = 7196;
		test_user.name = "Mackan".to_string();


		return test_user;
	}

	#[test]
	fn replace_sender_variables_sname_test() {
		let test_user = get_test_user();
		assert_eq!(replace_sender_variables("{sname}".to_string(), &test_user), test_user.name.to_string());
	}

	#[test]
	fn replace_sender_variables_sid_test() {
		let test_user = get_test_user();
		assert_eq!(replace_sender_variables("{sid}".to_string(), &test_user), test_user.id.to_string());
	}

	#[test]
	fn replace_sender_variables_sdiscrim_test() {
		let test_user = get_test_user();
		assert_eq!(replace_sender_variables("{sdiscrim}".to_string(), &test_user), test_user.discriminator.to_string());
	}


	#[test]
	fn replace_sender_variables_sbot_test() {
		let test_user = get_test_user();
		assert_eq!(replace_sender_variables("{sbot}".to_string(), &test_user), test_user.bot.to_string());
	}


	#[test]
	fn replace_sender_variables_sender_test() {
		let test_user = get_test_user();
		assert_eq!(replace_sender_variables("{sender}".to_string(), &test_user), format!("<@{}>", test_user.id.to_string()));
	}
}