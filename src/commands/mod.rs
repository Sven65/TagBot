pub mod framework;
#[macro_use]
pub mod commands;

use commands::ping2::ping2;
use commands::add::add;

use self::commands::add::{add_tag_handle_modal, add_options_creator};
use self::commands::delete::{delete, delete_options_creator};
use self::commands::edit::{edit, edit_options_creator, edit_tag_handle_modal};
use self::commands::glist::glist;
use self::commands::list::list;
use self::commands::raw::{raw, raw_options_creator};
use self::commands::stealtag::{stealtag, stealtag_options_creator};
use self::commands::tag::{tag, tag_options_creator};
use self::commands::tagowner::{tagowner, tagowner_options_creator};
use self::framework::{COMMAND_INDEX};


#[allow(dead_code)]
pub async fn init_commands() {
	COMMAND_INDEX.lock().await.register_command("ping2", |data, ctx| Box::pin(ping2(data, ctx)), Some("With desc"), None, false, None, None).await;
	COMMAND_INDEX.lock().await.register_command("add", |data, ctx| Box::pin(add(data, ctx)), Some("Adds tags"), Some(add_options_creator), true, Some(|interaction, ctx| Box::pin(add_tag_handle_modal(interaction, ctx))), None).await;
	COMMAND_INDEX.lock().await.register_command("delete", |data, ctx| Box::pin(delete(data, ctx)), Some("Deletes a tag"), Some(delete_options_creator), false, None, None).await;
	COMMAND_INDEX.lock().await.register_command("edit", |data, ctx| Box::pin(edit(data, ctx)), Some("Edits a tag"), Some(edit_options_creator), false, Some(|interaction, ctx| Box::pin(edit_tag_handle_modal(interaction, ctx))), None).await;

	COMMAND_INDEX.lock().await.register_command("glist", |data, ctx| Box::pin(glist(data, ctx)), Some("Gives a list of all tags"), None, true, None, None).await;
	COMMAND_INDEX.lock().await.register_command("list", |data, ctx| Box::pin(list(data, ctx)), Some("Gives a list of all tags you own"), None, true, None, None).await;
	COMMAND_INDEX.lock().await.register_command("raw", |data, ctx| Box::pin(raw(data, ctx)), Some("Sends the raw data of a tag"), Some(raw_options_creator), true, None, None).await;

	COMMAND_INDEX.lock().await.register_command("stealtag", |data, ctx| Box::pin(stealtag(data, ctx)), Some("Steals a tag"), Some(stealtag_options_creator), false, None, None).await;
	
	COMMAND_INDEX.lock().await.register_command("tagowner", |data, ctx| Box::pin(tagowner(data, ctx)), Some("Shows who owns a tag"), Some(tagowner_options_creator), false, None, None).await;
	
	COMMAND_INDEX.lock().await.register_command("tag", |data, ctx| Box::pin(tag(data, ctx)), Some("Executes a tag"), Some(tag_options_creator), false, None, None).await;

}
