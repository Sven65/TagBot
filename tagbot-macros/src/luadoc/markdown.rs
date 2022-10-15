use std::io::BufWriter;
use std::io::Write;
use std::vec::Vec;

use super::document::Document;
use super::document::Method;

pub fn write_method(stream: &mut Vec<u8>, name: &String, method: &Method) {
	let returns = match &method.returns {
		super::comments::Annotation::Return(ret) => Some(ret),
		_ => None,
	};

	if returns.is_some() {
		writeln!(stream, "## {} -> {}\n", name, returns.unwrap().typ).unwrap();
	} else {
		writeln!(stream, "## {}\n", name).unwrap();
	}

	let desc: Vec<String> = method
		.desc
		.iter()
		.filter_map(|desc| match desc {
			super::comments::Annotation::Description(desc) => Some(desc.to_string()),
			_ => None,
		})
		.collect::<Vec<String>>();

	writeln!(stream, "{}\n", desc.join("\n")).unwrap();
}

pub fn generate_markdown(doc: &Document) -> Vec<String> {
	let mut stream = Vec::new();

	// let bytes = stream.into_inner().unwrap();
	// let string = String::from_utf8(bytes).unwrap();

	writeln!(&mut stream, "# {}\n", doc.title.title).unwrap();
	doc.title.note.iter().for_each(|note| {
		writeln!(&mut stream, "{}", note).unwrap();
	});

	writeln!(&mut stream, "# Attributes").unwrap();

	writeln!(&mut stream, "# Methods\n").unwrap();

	doc.methods
		.iter()
		.for_each(|(name, method)| write_method(&mut stream, name, method));

	writeln!(&mut stream, "# Operators").unwrap();

	let string = String::from_utf8(stream).unwrap();

	println!("Generated\n{}", string);

	vec![]
}
