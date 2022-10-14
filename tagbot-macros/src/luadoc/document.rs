#[derive(Debug)]
pub struct Method {}

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
	pub methods: Vec<Method>,
	/// Attributes of the class
	pub attributes: Vec<Attribute>,
	/// Operators that the class supports
	pub operators: Vec<Operator>,
}

impl Document {
	pub fn new() -> Self {
		Self {
			title: DocTitle::new(),
			methods: Vec::new(),
			attributes: Vec::new(),
			operators: Vec::new(),
		}
	}

	pub fn set_attrib() {}
}
