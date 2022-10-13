pub mod framework;
#[macro_use]
#[allow(clippy::module_inception)]
pub mod commands;

use commands::add::add;
use commands::ping2::ping2;

use self::commands::add::{add_options_creator, add_tag_handle_modal};
use self::commands::delete::{delete, delete_options_creator};
use self::commands::edit::{edit, edit_options_creator, edit_tag_handle_modal};
use self::commands::glist::glist;
use self::commands::list::list;
use self::commands::raw::{raw, raw_options_creator};
use self::commands::stealtag::{stealtag, stealtag_options_creator};
use self::commands::tag::{tag, tag_options_creator};
use self::commands::tagowner::{tagowner, tagowner_options_creator};
use self::framework::{RegistrationMeta, COMMAND_INDEX};

#[allow(dead_code)]
#[rustfmt::skip]
pub async fn init_commands() {
	COMMAND_INDEX.lock().await.register_command("ping2", RegistrationMeta  { f: |data, ctx| Box::pin(ping2(data, ctx)), desc: Some("With desc".to_string()), option_creator: None, sends_message: false, modal_handler: None, component_handler: None }).await;
	COMMAND_INDEX.lock().await.register_command("add", RegistrationMeta    { f: |data, ctx| Box::pin(add(data, ctx)), desc: Some("Adds tags".to_string()), option_creator: Some(add_options_creator), sends_message: true, modal_handler: Some(|interaction, ctx| Box::pin(add_tag_handle_modal(interaction, ctx))), component_handler: None }).await;
	COMMAND_INDEX.lock().await.register_command("delete", RegistrationMeta { f: |data, ctx| Box::pin(delete(data, ctx)), desc: Some("Deletes a tag".to_string()), option_creator: Some(delete_options_creator), sends_message: false, modal_handler: None, component_handler: None}).await;
	COMMAND_INDEX.lock().await.register_command("edit", RegistrationMeta   { f: |data, ctx| Box::pin(edit(data, ctx)), desc: Some("Edits a tag".to_string()), option_creator: Some(edit_options_creator), sends_message: false, modal_handler: Some(|interaction, ctx| Box::pin(edit_tag_handle_modal(interaction, ctx))), component_handler: None}).await;

	COMMAND_INDEX.lock().await.register_command("glist", RegistrationMeta  { f: |data, ctx| Box::pin(glist(data, ctx)), desc: Some("Gives a list of all tags".to_string()), option_creator: None, sends_message: true, modal_handler: None, component_handler: None}).await;
	COMMAND_INDEX.lock().await.register_command("list", RegistrationMeta   { f: |data, ctx| Box::pin(list(data, ctx)), desc: Some("Gives a list of all tags you own".to_string()), option_creator: None, sends_message: true, modal_handler: None, component_handler: None}).await;
	COMMAND_INDEX.lock().await.register_command("raw", RegistrationMeta    { f: |data, ctx| Box::pin(raw(data, ctx)), desc: Some("Sends the raw data of a tag".to_string()), option_creator: Some(raw_options_creator), sends_message: true, modal_handler: None, component_handler: None}).await;

	COMMAND_INDEX.lock().await.register_command("stealtag", RegistrationMeta { f: |data, ctx| Box::pin(stealtag(data, ctx)), desc: Some("Steals a tag".to_string()), option_creator: Some(stealtag_options_creator), sends_message: false, modal_handler: None, component_handler: None}).await;
	
	COMMAND_INDEX.lock().await.register_command("tagowner", RegistrationMeta { f: |data, ctx| Box::pin(tagowner(data, ctx)), desc: Some("Shows who owns a tag".to_string()), option_creator: Some(tagowner_options_creator), sends_message: false, modal_handler: None, component_handler: None}).await;
	
	COMMAND_INDEX.lock().await.register_command("tag", RegistrationMeta { f: |data, ctx| Box::pin(tag(data, ctx)), desc: Some("Executes a tag".to_string()), option_creator: Some(tag_options_creator), sends_message: true, modal_handler:  None, component_handler: None}).await;
}
