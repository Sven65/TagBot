use crate::util::paths::Paths;

use super::registry::LUA_MODULE_INDEX;

fn resolve_path(module_path: &str) -> String {
	let paths = Paths::new();

	let mut path = paths.prefix.clone();

	path.push("data");
	path.push("lua");
	path.push(module_path);

	// println!("Path {:?}", path.clone());

	let path = path.to_str().unwrap().to_string();

	return path;
}

pub async fn init_modules() {
	LUA_MODULE_INDEX.lock().unwrap().register_module_file("util", &resolve_path("util.lua"));
}

