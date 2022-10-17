use std::collections::HashMap;

use super::comments::Annotation;

#[derive(Debug)]
pub struct Method {
	pub desc: Vec<Annotation>,
	pub params: HashMap<String, Annotation>,
	pub returns: Annotation,
}

impl From<&Vec<Annotation>> for Method {
	fn from(annotations: &Vec<Annotation>) -> Self {
		let mut method =
			Self { desc: Vec::new(), params: HashMap::new(), returns: Annotation::None };

		method.desc = annotations
			.iter()
			.cloned()
			.filter_map(|annot| match annot {
				Annotation::Description(_) => Some(annot),
				_ => None,
			})
			.collect::<Vec<Annotation>>();

		method.params = annotations
			.iter()
			.cloned()
			.filter_map(|annot| match annot.to_owned() {
				Annotation::Param(param) => Some((annot, param)),
				_ => None,
			})
			.map(|(annot, param)| (param.param, annot))
			.collect::<HashMap<String, Annotation>>();

		method.returns = annotations
			.iter()
			.cloned()
			.filter_map(|annot| match annot {
				Annotation::Return(_) => Some(annot),
				_ => None,
			})
			.collect::<Vec<Annotation>>()
			.get(0)
			.unwrap()
			.to_owned();

		method
	}
}

#[derive(Debug)]
pub struct Attribute {
	/// The name of the attribute
	/// `"my_value" => {}` would be "my_value"
	pub name: String,
	/// The type that the attribute returns
	/// Parsed from conversion functions in [`tagbot::tags::lua::lua_modules::rs_lua::types::utils::functions`]
	pub typ: String,
	/// If the conversion can return [`rlua::Value::Nil`]
	/// Parsed from option converters
	pub optional: bool,
}

#[derive(Debug)]
pub struct DocTitle {
	pub title: String,
	pub note: Vec<String>,
}

impl DocTitle {
	pub fn new() -> Self {
		Self { title: "".to_string(), note: Vec::new() }
	}
}

#[derive(Debug)]
pub struct Operator {}

#[derive(Debug)]
pub struct Document {
	/// Title of the class
	pub title: DocTitle,
	/// Methods the class hass
	pub methods: HashMap<String, Method>,
	/// Attributes of the class
	pub attributes: Vec<Attribute>,
	/// Operators that the class supports
	pub operators: Vec<Operator>,

	/// If the doc has a requireable module.
	pub requireable: bool,

	pub requireable_as: Option<String>,

	/// Functions exposed through requireable
	pub requireable_functions: HashMap<String, Method>,
}

impl Document {
	pub fn new() -> Self {
		Self {
			title: DocTitle::new(),
			methods: HashMap::new(),
			attributes: Vec::new(),
			operators: Vec::new(),
			requireable: false,
			requireable_as: None,
			requireable_functions: HashMap::new(),
		}
	}
}
