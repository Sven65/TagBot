use dotenv::dotenv;
use std::{env};
use reql::{r, cmd::connect::Options, Session};
use lazy_static::{lazy_static};
use tokio::runtime::Handle;

use crate::infoln;

#[derive(Clone)]
pub struct RethinkDB {
	pub session: Option<Session>,
}

impl RethinkDB {
	async fn init_connection (&mut self) -> Result<&Session, reql::Error> {
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

		infoln!("Connected to RethinkDB.");

		return Ok(self.session.as_ref().unwrap());
	}

	fn set_connection(&mut self, conn: Option<Session>) {
		self.session = conn;
	}

	pub async fn getConnection(&self) -> Option<&Session> {
		return self.session.as_ref();
	}

	pub fn new() -> Self {
		let mut rdb: RethinkDB = RethinkDB {
			session: None,
		};

		let handle = Handle::current();

		handle.enter();

		futures::executor::block_on(rdb.init_connection());



		return rdb;
	}
}

lazy_static! {
	pub static ref RDB: RethinkDB = RethinkDB::new();
}
