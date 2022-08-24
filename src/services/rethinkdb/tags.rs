use futures::TryStreamExt;
use reql::{r, Session, types::WriteStatus};
use serde::{Serialize, Deserialize};

use super::rethinkdb::RDB;

macro_rules! create_error {
	($($args:tt)*) => {
		Err(reql::Error::from(std::io::Error::new(std::io::ErrorKind::Other, $($args)*)));
	};
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Tag {
	pub id: String,
	pub content: String,
	pub owner: String,
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
	/// Adds a tag to the tags table.
	/// 
	/// # Arguments
	/// 
	/// * `tag_name` - The name of the tag to insert. Automatically converted to lowercase.
	/// * `content` - The content of the tag
	/// * `owner_id` - Snowflake of the tag owner
	pub async fn add_tag(tag_name: String, content: String, owner_id: String) -> Result<WriteStatus, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			return create_error!("Failed to create tag: Failed to get DB Connection.".to_string());
		}

		let connection = connection.unwrap();

		let tag = Tag::new (
			tag_name.to_lowercase(),
			content,
			owner_id,
		);


		let mut query = r.table("Tags").insert(tag).run::<&reql::Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			return Ok(result);
		}

		return create_error!("Failed to insert tag");

	}


	/// Edits a tag in the tags table.
	/// 
	/// # Arguments
	/// 
	/// * `tag_name` - The name of the tag to edit. Automatically converted to lowercase.
	/// * `content` - The new content of the tag
	/// * `owner_id` - Snowflake of the tag owner
	pub async fn edit_tag(tag_name: String, content: String, owner_id: String) -> Result<WriteStatus, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			return create_error!("Failed to edit tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let tag = Tag::new (
			tag_name.to_lowercase(),
			content,
			owner_id,
		);


		let mut query = r.table("Tags").get(tag_name.to_lowercase()).update(tag).run::<&reql::Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			return Ok(result);
		}

		return create_error!("Failed to update tag");

	}


	/// Gets a tag from the tags table, if it exists.
	/// 
	/// # Arguments
	/// 
	/// * `tag_name` - The name of the tag to get. Automatically converted to lowercase.
	pub async fn get_tag(tag_name: String) -> Result<Tag, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			return create_error!("Failed to get tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let mut query = r.table("Tags").get(tag_name.to_lowercase()).run::<&Session, Tag>(connection);

		if let Some(result) = query.try_next().await? {
			println!("Result {:?}", result);
			return Ok(result);
		}
	

		return create_error!("Failed to get tag");
	}

	/// Deletes a tag from the tags table, if it exists.
	/// 
	/// # Arguments
	/// 
	/// * `tag_name` - The name of the tag to delete. Automatically converted to lowercase.
	pub async fn delete_tag(tag_name: String) -> Result<WriteStatus, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			return create_error!("Failed to get tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let delete_options = reql::cmd::delete::Options::new();

		let mut query = r.table("Tags").get(tag_name.to_lowercase()).delete(delete_options).run::<&Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			println!("Result {:?}", result);
			return Ok(result);
		}
	

		return create_error!("Failed to get tag");
	}

	pub async fn get_all() -> Result<Vec<Tag>, reql::Error> {
		let connection = RDB.getConnection().await;

		if connection.is_none() {
			return create_error!("Failed to get all tags: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();


		let mut query = r.table("Tags").run::<&Session, Tag>(connection);

		let mut tags: Vec<Tag> = Vec::new();

		while let Some(result) = query.try_next().await? {
			println!("Result {:?}", result);

			tags.push(result);
		}

		return Ok(tags);
	}
}