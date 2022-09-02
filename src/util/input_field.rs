use serenity::model::{application::component::{ActionRow, ActionRowComponent}, prelude::component::{InputText, SelectMenu}};

pub trait FindInput {
	fn find_input(&self, name: &str) -> Option<&InputText>;
	fn find_select(&self, name: &str) -> Option<&SelectMenu>;
}

impl FindInput for Vec<ActionRow> {
	/// Finds an input by custom id
	/// 
	/// # Arguments
	/// 
	/// * `name` - The custom id of the field to find
    fn find_input(&self, name: &str) -> Option<&InputText> {
		let mut res: Option<&InputText> = None;

		self.iter().for_each(|item| {
			let components = &item.components;

			components.iter().for_each(|component| {
				match component {
					ActionRowComponent::InputText(component) => {
						if component.custom_id == name {
							res = Some(component);
						}
					},
					&_ => {},
				}
			});
		});

		return res;
    }

	/// Finds a select menu by custom id
	/// 
	/// # Arguments
	/// 
	/// * `name` - The custom id of the select menu to find
	fn find_select(&self, name: &str) -> Option<&SelectMenu> {
		let mut res: Option<&SelectMenu> = None;

		self.iter().for_each(|item| {
			let components = &item.components;

			components.iter().for_each(|component| {
				match component {
					ActionRowComponent::SelectMenu(component) => {
						if component.custom_id.as_ref().unwrap() == name {
							res = Some(component);
						}
					},
					&_ => {},
				}
			});
		});

		return res;
	}
}