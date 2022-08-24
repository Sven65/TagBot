use futures::TryStreamExt;
use reql::{r, Error, Session};
use serde::{Serialize, Deserialize};

use super::rethinkdb::RDB;

#[derive(Serialize, Debug, Deserialize)]
pub struct Tag {
	pub id: String,
	pub content: String,
	pub owner: String,
}

impl reql::cmd::default::Arg for Tag {
    fn arg(self) -> reql::cmd::Arg<()> {
		self.arg()
    }
}

impl Tag {
	pub fn new (id: String, content: String, owner: String) -> Self {
		return Tag {
			id: id,
			content: content,
			owner: owner,
		}
	}

	// pub fn empty() -> Self {
	// 	return Tag { id: None, content: None, owner: None }
	// }


	// pub fn is_empty(&self) -> bool {
	// 	return self.id.is_none() && self.content.is_none() && self.owner.is_none();
	// }


}

pub struct TagsTable {}

impl TagsTable {
	/// Gets a tag from the tags table, if it exists.
	/// 
	/// # Arguments
	/// 
	/// * `tag_name` - The name of the tag to get
	pub async fn get_tag(tag_name: String) -> Result<Tag, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			panic!("Failed to get tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let mut query = r.table("Tags").get(tag_name).run::<&Session, Tag>(connection);

		if let Some(result) = query.try_next().await? {
			println!("Result {:?}", result);
			return Ok(result);
		}
	

		return Err(reql::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get tag")));
	}

	/// Deletes a tag from the tags table, if it exists.
	/// 
	/// # Arguments
	/// 
	/// * `tag_name` - The name of the tag to delete
	pub async fn delete_tag(tag_name: String) -> Result<reql::types::WriteStatus, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			panic!("Failed to get tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let delete_options = reql::cmd::delete::Options::new();

		let mut query = r.table("Tags").get(tag_name).delete(delete_options).run::<&Session, reql::types::WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			println!("Result {:?}", result);
			return Ok(result);
		}
	

		return Err(reql::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get tag")));
	}
}