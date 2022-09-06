use rlua::{ToLua, UserData};
use serenity::model::{prelude::interaction::application_command::ApplicationCommandInteraction, prelude::User};

// Basically stolen from serenity,
// but since rust can't implement traits for types the current crate doesn't own without 4 billion boilerplate wrappers, we do our own types
#[derive(Clone, Debug)]
pub struct TBUser(User);

impl TBUser {
	pub fn new(user: User) -> TBUser {
		TBUser(user)
	}
}



// impl ToLua<'_> for TBUser {
//     fn to_lua<'lua>(self, lua: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
// 		let user = self.0;

// 		let table = lua.create_table()?;

// 		table.set("id", user.id.to_string())?;

// 		Ok(rlua::Value::Table(table))
// 	}
// }

impl UserData for TBUser {

}