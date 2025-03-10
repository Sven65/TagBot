use futures::TryStreamExt;
use reql::{func, r, types::WriteStatus, Session};
use serde::{Deserialize, Serialize};

use crate::services::rethinkdb::rethinkdb::RDB;

macro_rules! create_error {
	($($args:tt)*) => {
		Err(reql::Error::from(std::io::Error::new(std::io::ErrorKind::Other, $($args)*)))
	};
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum TagType {
	Legacy,
	Lua,
	/// Reserved for tag types that are invalid
	Invalid,
}

impl std::fmt::Display for TagType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let name = match self {
			Self::Lua => "lua",
			Self::Legacy => "legacy",
			Self::Invalid => "invalid",
		};

		write!(f, "{}", name)
	}
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Tag {
	pub id: String,
	pub content: String,
	pub owner: String,
	pub tag_type: Option<TagType>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct OwnerTag {
	pub id: String,
	pub owner: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct ContentTag {
	pub id: String,
	pub content: String,
}

impl Tag {
	pub fn new(id: String, content: String, owner: String, tag_type: Option<TagType>) -> Self {
		Tag { id, content, owner, tag_type }
	}
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
	pub async fn add_tag(
		tag_name: String,
		content: String,
		owner_id: String,
		tag_type: Option<TagType>,
	) -> Result<WriteStatus, reql::Error> {
		let rdb = RDB.lock().await;
		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to create tag: Failed to get DB Connection.".to_string());
		}

		let connection = connection.unwrap();

		let tag = Tag::new(tag_name.to_lowercase(), content, owner_id, tag_type);

		let mut query = r
			.table("Tags")
			.insert(tag)
			.run::<&reql::Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			return Ok(result);
		}

		create_error!("Failed to insert tag")
	}

	/// Gets a tag from the tags table, if it exists.
	///
	/// # Arguments
	///
	/// * `tag_name` - The name of the tag to get. Automatically converted to lowercase.
	pub async fn get_tag(tag_name: String) -> Result<Tag, reql::Error> {
		let mut rdb_lock = RDB.lock().await;

		let rdb = rdb_lock.to_owned();

		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to get tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let mut query = r
			.table("Tags")
			.get(tag_name.to_lowercase())
			.run::<&Session, Tag>(connection);

		match query.try_next().await {
			Ok(Some(result)) => Ok(result),
			Ok(None) => create_error!("Failed to get tag"),
			Err(e) => {
				rdb_lock.init_connection().await?;

				Err(e)
			} // Propagate the error
		}
	}

	/// Deletes a tag from the tags table, if it exists.
	///
	/// # Arguments
	///
	/// * `tag_name` - The name of the tag to delete. Automatically converted to lowercase.
	pub async fn delete_tag(tag_name: String) -> Result<WriteStatus, reql::Error> {
		let rdb = RDB.lock().await;
		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to get tag: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let delete_options = reql::cmd::delete::Options::new();

		let mut query = r
			.table("Tags")
			.get(tag_name.to_lowercase())
			.delete(delete_options)
			.run::<&Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			return Ok(result);
		}

		create_error!("Failed to get tag")
	}

	/// Gets all tags in the database
	pub async fn get_all() -> Result<Vec<Tag>, reql::Error> {
		let rdb = RDB.lock().await;
		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to get all tags: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let mut query = r.table("Tags").run::<&Session, Tag>(connection);

		let mut tags: Vec<Tag> = Vec::new();

		while let Some(result) = query.try_next().await? {
			tags.push(result);
		}

		Ok(tags)
	}

	/// Gets all tags in the database owned by a user
	///
	/// # Arguments
	///
	/// * `owner_id` - The id of the user whose tags to get
	pub async fn get_all_by_owner(owner_id: String) -> Result<Vec<Tag>, reql::Error> {
		let rdb = RDB.lock().await;
		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to get user tags: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let mut query = r
			.table("Tags")
			.filter(func!(|doc| { doc.get_field("owner").eq(owner_id) }))
			.run::<&Session, Tag>(connection);

		let mut tags: Vec<Tag> = Vec::new();

		while let Some(result) = query.try_next().await? {
			tags.push(result);
		}

		Ok(tags)
	}

	/// Sets the owner of a tag
	///
	/// # Arguments
	///
	/// * `tag_name` - The id of the tag to change the owner of
	/// * `new_owner` - The id of the new tag owner
	pub async fn set_owner(
		tag_name: String,
		new_owner: String,
	) -> Result<WriteStatus, reql::Error> {
		let rdb = RDB.lock().await;
		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to set tag owner: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let new_tag = OwnerTag { id: tag_name.clone(), owner: new_owner };

		let mut query = r
			.table("Tags")
			.get(tag_name)
			.update(new_tag)
			.run::<&Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			return Ok(result);
		}

		create_error!("Failed to update tag owner")
	}

	/// Sets the content of a tag
	///
	/// # Arguments
	///
	/// * `tag_name` - The id of the tag to change the content of
	/// * `new_content` - The new content for the tag
	pub async fn set_content(
		tag_name: String,
		new_content: String,
	) -> Result<WriteStatus, reql::Error> {
		let rdb = RDB.lock().await;
		let connection = rdb.get_connection().await;

		if connection.is_none() {
			return create_error!("Failed to set tag content: Failed to get DB Connection.");
		}

		let connection = connection.unwrap();

		let new_tag = ContentTag { id: tag_name.clone(), content: new_content };

		let mut query = r
			.table("Tags")
			.get(tag_name)
			.update(new_tag)
			.run::<&Session, WriteStatus>(connection);

		if let Some(result) = query.try_next().await? {
			return Ok(result);
		}

		create_error!("Failed to update tag content")
	}
}
