use cat_loggr::{log_fatal, log_info};
use dotenv::dotenv;
use futures::lock::Mutex;
use lazy_static::lazy_static;
use reql::{cmd::connect::Options, r, Session};
use std::{env, sync::Arc};
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct RethinkDBOptions {
	host: String,
	port: u16,
	db_name: String,
	user: String,
	password: String,
}

#[derive(Clone)]
pub struct RethinkDB {
	pub connection_options: Option<RethinkDBOptions>,
	pub session: Option<Session>,
}

impl RethinkDB {
	pub fn get_options_as_connect(&self) -> Options {
		let rdb_opts = self.connection_options.clone().unwrap();
		Options::new()
			.host(rdb_opts.host)
			.port(rdb_opts.port)
			.db(rdb_opts.db_name)
			.user(rdb_opts.user)
			.password(rdb_opts.password)
	}

	pub fn init_options(&mut self) {
		dotenv().ok();

		let host = env::var("RETHINK_HOST").expect("Expected rethinkdb host to be present in env.");
		let port = env::var("RETHINK_PORT")
			.expect("Expected rethinkdb port to be present in env.")
			.parse::<u16>()
			.unwrap();
		let db_name =
			env::var("RETHINK_DB_NAME").expect("Expected rethinkdb db name to be present in env.");
		let user = env::var("RETHINK_USER").expect("Expected rethinkdb user to be present in env.");
		let password = env::var("RETHINK_PASSWORD")
			.expect("Expected rethinkdb password to be present in env.");

		self.connection_options = Some(RethinkDBOptions { host, port, db_name, user, password });
	}

	pub async fn init_connection(&mut self) -> Result<&Session, reql::Error> {
		let conn = r.connect(self.get_options_as_connect()).await?;

		self.session = Some(conn);

		log_info!("Connected to RethinkDB.");

		return Ok(self.session.as_ref().unwrap());
	}

	pub async fn get_connection(&self) -> Option<&Session> {
		let connection = self.session.as_ref();

		if connection.is_none() {
			println!("Connection to DB lost.");
		}

		connection
	}

	pub fn new() -> Self {
		let mut rdb: RethinkDB = RethinkDB { session: None, connection_options: None };

		rdb.init_options();

		let handle = Handle::current();

		#[allow(unused_must_use)]
		{
			handle.enter();
		}

		let res = futures::executor::block_on(rdb.init_connection());

		if res.is_err() {
			log_fatal!(
				"Failed to spawn blocker while creating connection: {:#?}",
				res.err()
			);
		}

		rdb
	}
}

lazy_static! {
	pub static ref RDB: Arc<Mutex<RethinkDB>> = Arc::new(Mutex::new(RethinkDB::new()));
}
