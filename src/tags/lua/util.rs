use rlua::Table;

pub fn dump_table(table: &Table) -> String {
	let mut result = String::new();
	for pair in table.clone().pairs::<String, String>() {
		match pair {
			Ok((key, value)) => {
				result.push_str(&format!("{}: {}\n", key, value));
			}
			Err(_) => continue, // Skip non-string pairs for simplicity
		}
	}
	result
}
