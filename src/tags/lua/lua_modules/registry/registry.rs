use std::{collections::HashMap, sync::Mutex, fs, io::ErrorKind};
use lazy_static::lazy_static;
use core::result::Result;
use std::io::Error;

#[derive(Clone)]
pub struct LuaModule {
	name: String,
	path: String,
}

#[derive(Clone)]
pub struct LuaModuleRegistry {
	pub modules: HashMap<String, LuaModule>,
}

impl LuaModuleRegistry {
	pub fn register_module_file(&mut self, name: &str, module_path: &str) {
		println!("Registering {} with path {}", name, module_path);

		let module = LuaModule {
			name: name.to_string(),
			path: module_path.to_string(),
		};

		self.modules.insert(name.to_string(), module);
	}

	pub fn has_module(&self, name: &str) -> bool {
		return self.modules.contains_key(&name.to_lowercase());
	}

	pub fn load_module_to_string(&self, name: &str) -> Result<String, Error> {
		let module_name = name.to_lowercase();

		if !self.has_module(name) {
			return Err(Error::new(ErrorKind::Other, format!("Module {} not found.", name)));
		}

		let lua_module = self.modules.get(&module_name);

		let lua_module = lua_module.unwrap();

		let contents = fs::read_to_string(lua_module.path.as_str()).expect("Unable to read file for module");

		Ok(contents)
	}
}


lazy_static! {
	pub static ref LUA_MODULE_INDEX: Mutex<LuaModuleRegistry> = Mutex::new(LuaModuleRegistry {
		modules: HashMap::new(),
	});
}

