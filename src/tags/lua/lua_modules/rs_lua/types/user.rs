// Wraps a serenity user as a lua variable

use rlua::{ToLua, UserData, MetaMethod, Value};
use serenity::model::{prelude::User};

#[derive(Clone, Debug)]
pub struct TBUser(User);

impl TBUser {
	pub fn new(user: User) -> TBUser {
		TBUser(user)
	}
}


// This looks wild, but it's needed for indexing lol
impl UserData for TBUser {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&"id" => this.0.id.to_string().to_lua(ctx)?,
				&"name" => this.0.name.clone().to_lua(ctx)?,
				&"avatar" => {
					let val = this.0.avatar.as_ref();
					if val.is_none() {
						Value::Nil
					} else {
						val.unwrap().clone().to_lua(ctx)?
					}
				},
				&"banner" => {
					let val = this.0.banner.as_ref();
					if val.is_none() {
						Value::Nil
					} else {
						val.unwrap().clone().to_lua(ctx)?
					}
				},
				&"bot" => this.0.bot.to_lua(ctx)?,
				&"discriminator" => this.0.discriminator.to_lua(ctx)?,
				&"tag" => this.0.tag().to_lua(ctx)?,
				&_ => Value::Nil,
			})
		});
	}
}

#[cfg(test)]
mod tests {
    use rlua::{Lua};
    use serenity::model::user::User;
	use test_case::test_case;

    use super::TBUser;

	fn create_user() -> TBUser {
		TBUser::new(User::default())
	}

	#[test_case("id", "210" ; "Gets the correct ID")]
	#[test_case("name", "test" ; "Gets the correct name")]
	#[test_case("discriminator", "1432" ; "Gets the correct discriminator")]
	#[test_case("tag", "test#1432" ; "Gets the correct tag")]
	fn get_str(param: &str, expected: &str) {
		Lua::new().context(|lua| {
			let userdata = lua.create_userdata(create_user()).unwrap();
			let globals = lua.globals();
			globals.set("userdata", userdata).unwrap();

			let data = lua.load(
				format!(r#"
					return userdata.{}
				"#, param).as_str(),
			).eval::<String>().unwrap();

			assert_eq!(data, expected)
		})
	}

	#[test_case("bot", true ; "Gets that the user is a bot")]
	fn get_bool(param: &str, expected: bool) {
		Lua::new().context(|lua| {
			let userdata = lua.create_userdata(create_user()).unwrap();
			let globals = lua.globals();
			globals.set("userdata", userdata).unwrap();

			let data = lua.load(
				format!(r#"
					return userdata.{}
				"#, param).as_str(),
			).eval::<bool>().unwrap();

			assert_eq!(data, expected)
		})
	}
}
