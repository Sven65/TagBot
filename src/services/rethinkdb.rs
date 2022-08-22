use dotenv::dotenv;
use std::{env, sync::Mutex};
use reql::{r, cmd::connect::Options, Session};
use lazy_static::{lazy_static};
use serde::Serialize;

#[derive(Serialize)]
pub struct Tag<'a> {
	id: &'a str,
	content: &'a str,
	owner: &'a str
}

pub struct RethinkDB {
	pub session: Option<Session>,
}

impl RethinkDB {
	pub async fn getConnection(&mut self) -> Option<&Session> {
		if self.session.is_some() {
			return self.session.as_ref();
		}

		self.init().await;

		return self.session.as_ref();
	}
	
	pub async fn init (&mut self) -> Result<bool, reql::Error> {
		if self.session.is_some() {
			return Ok(true);
		}

		dotenv().ok();

		let host = env::var("RETHINK_HOST").expect("Expected rethinkdb host to be present in env.");
		let port = env::var("RETHINK_PORT").expect("Expected rethinkdb port to be present in env.").parse::<u16>().unwrap();
		let db_name = env::var("RETHINK_DB_NAME").expect("Expected rethinkdb db name to be present in env.");
		let user = env::var("RETHINK_USER").expect("Expected rethinkdb user to be present in env.");
		let password = env::var("RETHINK_PASSWORD").expect("Expected rethinkdb password to be present in env.");

		let options = Options::new()
			.host(host)
			.port(port)
			.db(db_name)
			.user(user)
			.password(password);

		let conn = r.connect(options).await?;

		self.session = Some(conn);

		return Ok(true);
	}
}

lazy_static! {
	pub static ref RDB: Mutex<RethinkDB> = Mutex::new(RethinkDB {
		session: None,
	});
}