use rlua::Lua;

pub struct Sender {
	pub id: String,
	pub name: String,
	pub discriminator: String,
	pub bot: bool,
}

impl Sender {
	pub fn new() {

	}

	pub fn to_mention(&self) -> String {
		format!("<@{}>", self.id)
	}
}

// pub struct SenderModule {

// }

// impl SenderModule {
// 	pub fn to_lua() {
// 		Lua::from(Sender::new())
// 	}
// }