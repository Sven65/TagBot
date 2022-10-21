use std::io::Write;
use std::vec::Vec;

use super::comments::Annotation;
use super::document::Attribute;
use super::document::Document;
use super::document::Method;

// todo: method return table values

fn generate_method_signature(name: &String, method: &Method) -> String {
	let params: String = method
		.params
		.iter()
		.map(|(param_name, annotation)| param_name.to_string())
		.collect::<Vec<String>>()
		.join(",");

	format!("{}({})", name, params)
}

pub fn write_annotation(stream: &mut Vec<u8>, annotation: &Annotation, param_name: Option<String>) {
	match annotation {
		Annotation::Param(val) => writeln!(
			stream,
			"- {} :: {} | {}",
			param_name.unwrap_or(val.param.to_string()),
			val.typ,
			val.desc
		)
		.unwrap(),
		Annotation::Table(val) => val
			.children
			.iter()
			.for_each(|child| write_annotation(stream, child, None)),
		_ => {}
	}
}

fn write_returns(stream: &mut Vec<u8>, annotation: &Annotation) {
	match &annotation {
		super::comments::Annotation::Return(ret) => {
			writeln!(stream, "- :: {} | {}\n", ret.typ, ret.desc).unwrap()
		}
		super::comments::Annotation::ReturnParam(ret) => {
			writeln!(stream, "- {} :: {} | {}\n", ret.param, ret.typ, ret.desc).unwrap()
		}
		super::comments::Annotation::ReturnTable(ret) => {
			writeln!(stream, "- {} :: {} | {}\n", ret.param, ret.typ, ret.desc).unwrap();

			ret.children
				.iter()
				.for_each(|child| write_returns(stream, child));
		}
		_ => {}
	};
}

#[rustfmt::skip]
pub fn write_method(stream: &mut Vec<u8>, name: &String, method: &Method) {
	let returns = match &method.returns {
		super::comments::Annotation::Return(ret) => Some((ret.typ.clone(), ret.desc.clone())),
		super::comments::Annotation::ReturnParam(ret) => Some((ret.typ.clone(), ret.desc.clone())),
		super::comments::Annotation::ReturnTable(ret) => Some((ret.typ.clone(), ret.desc.clone())),
		_ => None,
	};

	let method_sig = generate_method_signature(name, method);

	if returns.is_some() {
		writeln!(stream, "## {} -> {}\n", method_sig, returns.as_ref().unwrap().0).unwrap();
	} else {
		writeln!(stream, "## {}\n", method_sig).unwrap();
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

	if method.params.len() > 0 {
		writeln!(stream, "### Params").unwrap();

		method.params.iter().for_each(|(param_name, param)| {
			write_annotation(stream, param, Some(param_name.to_string()))
		})
	}

	writeln!(stream, "\n").unwrap();

	if returns.is_some() {
		writeln!(stream, "### Return Values").unwrap();
		write_returns(stream, &method.returns);
	}
}

fn write_attribute(stream: &mut Vec<u8>, attribute: &Attribute) {
	writeln!(
		stream,
		"- {} :: {}{}",
		attribute.name,
		attribute.typ,
		match attribute.optional {
			true => "?",
			false => "",
		}
	)
	.unwrap();
}

pub fn generate_markdown(doc: &Document) -> String {
	let mut stream = Vec::new();

	// let bytes = stream.into_inner().unwrap();
	// let string = String::from_utf8(bytes).unwrap();

	writeln!(&mut stream, "# {}\n", doc.title.title).unwrap();
	doc.title.note.iter().for_each(|note| {
		writeln!(&mut stream, "{}", note).unwrap();
	});

	if doc.attributes.len() > 0 {
		writeln!(&mut stream, "# Attributes").unwrap();

		doc.attributes
			.iter()
			.for_each(|attribute| write_attribute(&mut stream, attribute));
	}

	if doc.methods.len() > 0 {
		writeln!(&mut stream, "# Methods\n").unwrap();

		doc.methods
			.iter()
			.for_each(|(name, method)| write_method(&mut stream, name, method));
	}

	if doc.operators.len() > 0 {
		writeln!(&mut stream, "# Operators").unwrap();
	}

	if doc.requireable {
		writeln!(&mut stream, "# Requireable\n").unwrap();
		writeln!(
			&mut stream,
			"This module is requireable as `{}`.\n",
			doc.requireable_as.as_ref().unwrap(),
		)
		.unwrap();

		if doc.requireable_functions.len() > 0 {
			writeln!(&mut stream, "## Functions\n").unwrap();

			doc.requireable_functions
				.iter()
				.for_each(|(name, method)| write_method(&mut stream, name, method));
		}
	}

	let string = String::from_utf8(stream).unwrap();

	string
}
