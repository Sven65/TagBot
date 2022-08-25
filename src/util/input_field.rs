use serenity::model::{application::component::{ActionRow, ActionRowComponent}, prelude::component::InputText};

pub trait FindInput {
	fn find_input(&self, name: &str) -> Option<&InputText>;
}

impl FindInput for Vec<ActionRow> {
    fn find_input(&self, name: &str) -> Option<&InputText> {
		let mut res: Option<&InputText> = None;

		self.iter().for_each(|item| {
			let components = &item.components;

			components.iter().for_each(|component| {
				match component {
					ActionRowComponent::InputText(component) => {
						println!("Component id {}, wanted = {}", component.custom_id, name);
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
}