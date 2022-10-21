use cat_loggr::log_debug;
use core::result::Result;
use lazy_static::lazy_static;
use rlua::{Context, Value};
use std::io::Error;
use std::{collections::HashMap, fs, io::ErrorKind, sync::Mutex};

type RustModuleFn = fn(context: Context) -> Value;

#[derive(Clone, PartialEq, Eq)]
pub enum LuaModuleType {
	LuaFile,
	RustSource,
}

#[derive(Clone)]
pub struct LuaModule {
	module_type: LuaModuleType,
	path: Option<String>,
	func: Option<RustModuleFn>,
}

#[derive(Clone)]
pub struct LuaModuleRegistry {
	pub modules: HashMap<String, LuaModule>,
}

impl LuaModuleRegistry {
	/// Registers a lua require module to load from disk
	///
	/// # Arguments
	///
	/// * `name` - The name of the module, used in `require()`
	/// * `module_path` - The path of the file on disk
	pub fn register_module_file(&mut self, name: &str, module_path: &str) {
		log_debug!("Registering module \"{}\" with path {}", name, module_path);

		let module = LuaModule {
			module_type: LuaModuleType::LuaFile,
			path: Some(module_path.to_string()),
			func: None,
		};

		self.modules.insert(name.to_string(), module);
	}

	/// Checks if the registry has a module
	///
	/// # Arguments
	///
	/// * `name` - Name of the module to check for
	pub fn has_module(&self, name: &str) -> bool {
		self.modules.contains_key(&name.to_lowercase())
	}

	pub fn get_module(&self, name: &str) -> Result<&LuaModule, Error> {
		let module = self.modules.get(&name.to_lowercase());

		if module.is_none() {
			return Err(Error::new(
				ErrorKind::Other,
				format!("Module {} not found.", name),
			));
		}

		Ok(module.unwrap())
	}

	/// Reads a registered module file from disk and returns it as a string
	///
	/// # Arguments
	///
	/// * `name` - The name of the module to read
	pub fn load_module_to_string(&self, name: &str) -> Result<String, Error> {
		let module_name = name.to_lowercase();

		if !self.has_module(name) {
			return Err(Error::new(
				ErrorKind::Other,
				format!("Module {} not found.", name),
			));
		}

		let lua_module = self.modules.get(&module_name);

		let lua_module = lua_module.unwrap();

		if lua_module.module_type != LuaModuleType::LuaFile {
			return Err(Error::new(
				ErrorKind::Other,
				format!("Module {} is not a loadable file.", name),
			));
		}

		let path = lua_module.path.as_ref().unwrap();

		let contents = fs::read_to_string(path.as_str())
			.expect(format!("Unable to read file for module path {:#?}", path).as_str());

		Ok(contents)
	}

	pub fn load_lua_module<'lua>(
		&self,
		name: &str,
		ctx: Context<'lua>,
	) -> Result<Value<'lua>, Error> {
		let contents = self.load_module_to_string(name)?;

		let chunk = ctx.load(contents.as_str());

		let result = chunk.eval::<Value>();

		if result.is_err() {
			return Err(Error::new(ErrorKind::Other, result.err().unwrap()));
		}

		let final_result = result.unwrap();

		Ok(final_result)
	}

	/// Registers a lua require module with a rust struct
	///
	/// # Arguments
	///
	/// * `name` - The name of the module, used in `require()`
	pub fn register_rust_module(&mut self, name: &str, f: RustModuleFn) {
		log_debug!("Registering rust lua module \"{}\"", name);

		let module =
			LuaModule { module_type: LuaModuleType::RustSource, path: None, func: Some(f) };

		self.modules.insert(name.to_string(), module);
	}

	pub fn load_rust_module<'lua>(
		&self,
		name: &str,
		ctx: Context<'lua>,
	) -> Result<Value<'lua>, Error> {
		let name = name.to_lowercase();
		let name = name.as_str();

		if !self.has_module(name) {
			return Err(Error::new(
				ErrorKind::Other,
				format!("Module {} not found.", &name),
			));
		}

		let module = self.modules.get(name);

		let module = module.unwrap();

		if module.module_type != LuaModuleType::RustSource {
			return Err(Error::new(
				ErrorKind::Other,
				format!("Module {} is not a native module.", name),
			));
		}

		if module.func.is_none() {
			return Err(Error::new(
				ErrorKind::Other,
				format!("Module {} does not provide implementation.", name),
			));
		}

		let func = module.func.unwrap();

		let result = func(ctx);

		Ok(result)
	}

	pub fn load_module<'lua>(&self, name: &str, ctx: Context<'lua>) -> Result<Value<'lua>, Error> {
		let module = self.get_module(name)?;

		if module.module_type == LuaModuleType::LuaFile {
			return self.load_lua_module(name, ctx);
		}

		self.load_rust_module(name, ctx)
	}
}

lazy_static! {
	pub static ref LUA_MODULE_INDEX: Mutex<LuaModuleRegistry> =
		Mutex::new(LuaModuleRegistry { modules: HashMap::new() });
}
