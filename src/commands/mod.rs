pub mod framework;
#[macro_use]
pub mod commands;

use commands::ping2::ping2;
use commands::add::add;

use self::commands::add::add_options_creator;
use self::commands::delete::{delete, delete_options_creator};
use self::commands::edit::{edit, edit_options_creator};
use self::commands::glist::glist;
use self::framework::{COMMAND_INDEX};


#[allow(dead_code)]
pub async fn init_commands() {
	COMMAND_INDEX.lock().await.register_command("ping2", |data, ctx| Box::pin(ping2(data, ctx)), Some("With desc"), None, false).await;
	COMMAND_INDEX.lock().await.register_command("add", |data, ctx| Box::pin(add(data, ctx)), Some("Adds tags"), Some(add_options_creator), false).await;
	COMMAND_INDEX.lock().await.register_command("delete", |data, ctx| Box::pin(delete(data, ctx)), Some("Deletes a tag"), Some(delete_options_creator), false).await;
	COMMAND_INDEX.lock().await.register_command("edit", |data, ctx| Box::pin(edit(data, ctx)), Some("Edits a tag"), Some(edit_options_creator), false).await;
	COMMAND_INDEX.lock().await.register_command("glist", |data, ctx| Box::pin(glist(data, ctx)), Some("Gives a list of all tags"), None, true).await;

}
