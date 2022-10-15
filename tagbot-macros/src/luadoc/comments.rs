use std::collections::HashMap;

use darling::ToTokens;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use regex::Regex;

lazy_static! {
	// Regex for parsing `@annotation` at start of lines
	static ref ANNOTATION_REGEX: Regex = Regex::new(r"^\s*@(?P<annotation>\w+)").unwrap();
	static ref PARAM_VALUE_REGEX: Regex = Regex::new(r"^\s*(?P<annotation>@.*\s?)\s?\{(?P<type>.*)\}\s?(?P<param>.*?)\s(?P<rest>.*)$").unwrap();
	static ref PARAM_REGEX: Regex = Regex::new(r"\s*(?P<annotation>@.*\s?)\s?\{(?P<type>.*)\}\s?\s(?P<rest>.*)$").unwrap();

	static ref SIMPLE_ANNOTATION_REGEX: Regex = Regex::new(r"(?P<annotation>@\w+)\s+(?P<rest>.*)").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParamAnnotation {
	/// The type of annotation (@[type])
	pub annotation_type: String,
	/// The parsed type ({type})
	pub typ: String,
	/// The description of the string
	pub desc: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParamValueAnnotation {
	/// The type of annotation (@[type])
	pub annotation_type: String,
	/// The parsed type ({type})
	pub typ: String,
	/// The description of the string
	pub desc: String,
	/// The param being documented
	pub param: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Annotation {
	// Function,
	Method,
	Param(ParamValueAnnotation),
	Return(ParamAnnotation),
	Description(String),
	None,
}

/// Gets the comments for a method call from a TokenStream and returns it as a HashMap<K, V> where
/// K is the name of the method being documented.
/// V is the comments that have been extracted
///
/// # Arguments
/// * `tokens` - The [`TokenStream`] to parse
///
fn get_doc_groups(tokens: TokenStream) -> HashMap<String, Vec<String>> {
	let ast: syn::ItemFn = syn::parse(tokens.clone()).unwrap();

	let method_calls: Vec<syn::ExprMethodCall> = ast
		.block
		.stmts
		.into_iter()
		.filter_map(|stmt| match stmt {
			syn::Stmt::Semi(expr, _) => match expr {
				syn::Expr::MethodCall(call) => Some(call),
				_ => None,
			},
			syn::Stmt::Expr(expr) => match expr {
				syn::Expr::MethodCall(call) => Some(call),
				_ => None,
			},
			_ => None,
		})
		.collect();

	let docs: Vec<(String, Vec<String>)> = method_calls
		.into_iter()
		.map(|call| {
			let method_name = call.args.first();

			let method = match method_name {
				Some(expr) => match expr {
					syn::Expr::Lit(lit) => match &lit.lit {
						syn::Lit::Str(str) => Some(str.value()),
						_ => None,
					},
					_ => None,
				},
				None => None,
			};

			(call.attrs, method)
		})
		.filter_map(|(attrs, method)| {
			if attrs.len() == 0 || method.is_none() {
				return None;
			}

			let comments = attrs
				.iter()
				.map(|attr| attr.tokens.clone())
				.flat_map(|token| token)
				.filter_map(|token| match token {
					proc_macro2::TokenTree::Literal(lit) => {
						let lit: syn::Lit = syn::parse2(lit.to_token_stream()).unwrap();

						match lit {
							syn::Lit::Str(str) => Some(str.value()),
							_ => None,
						}
					}
					_ => None,
				})
				.collect::<Vec<String>>();

			Some((method.unwrap(), comments))
		})
		.collect();

	docs.into_iter().collect()
}

/// Parses a simple annotation (`@annotation desc thing`)
///
/// Returns a tuple of (annotation, description)
///
/// # Arguments
/// * `line` - The line to parse
fn parse_simple_annotation(line: &String) -> Option<(String, String)> {
	let captures = SIMPLE_ANNOTATION_REGEX.captures(line.as_str());

	match captures {
		Some(cap) => Some((
			cap.name("annotation").unwrap().as_str().to_string(),
			cap.name("rest").unwrap().as_str().to_string(),
		)),
		None => None,
	}
}

/// Parses a param value annotation
///
/// # Arguments
/// * `line` - The line to parse
fn parse_param_value_annotation(line: &String) -> Option<ParamValueAnnotation> {
	let captures = PARAM_VALUE_REGEX.captures(line.as_str());

	match captures {
		Some(cap) => Some(ParamValueAnnotation {
			annotation_type: cap.name("annotation").unwrap().as_str().to_string(),
			typ: cap.name("type").unwrap().as_str().to_string(),
			param: cap.name("param").unwrap().as_str().to_string(),
			desc: cap.name("rest").unwrap().as_str().to_string(),
		}),
		None => None,
	}
}

/// Parses a parm annotation
///
/// # Arguments
/// * `line` - The line to parse
fn parse_param_annotation(line: &String) -> Option<ParamAnnotation> {
	let captures = PARAM_REGEX.captures(line.as_str());

	match captures {
		Some(cap) => Some(ParamAnnotation {
			annotation_type: cap.name("annotation").unwrap().as_str().to_string(),
			typ: cap.name("type").unwrap().as_str().to_string(),
			desc: cap.name("rest").unwrap().as_str().to_string(),
		}),
		None => None,
	}
}

/// Parses a @return annotation
///
/// # Arguments
/// * `line` - The line to parse
fn parse_return(line: &String) -> Annotation {
	let parsed = parse_param_annotation(line);

	if parsed.is_none() {
		panic!("Return parser could not execute.");
	}

	let parsed = parsed.unwrap();

	Annotation::Return(parsed)
}

/// Parses a @param annotation
///
/// # Arguments
/// * `line` - The line to parse
fn parse_param(line: &String) -> Annotation {
	let parsed = parse_param_value_annotation(line);

	if parsed.is_none() {
		panic!("Param parser could not execute.");
	}

	let parsed = parsed.unwrap();

	Annotation::Param(parsed)
}

/// Parses a `@desc` comment
///
/// # Arguments
/// * `line` - The line to parse
fn parse_desc(line: &String) -> Annotation {
	let parsed = parse_simple_annotation(line);

	if parsed.is_none() {
		panic!("Description parser could not execute.");
	}

	Annotation::Description(parsed.unwrap().1)
}

/// Parsed a comment string line into annotation
///
/// # Arguments
/// * `line` - The line to parse
fn parse_comment_line(line: &String) -> Option<Annotation> {
	let captures = ANNOTATION_REGEX.captures(line.as_str());

	if captures.is_none() {
		return None;
	}

	let captures = captures.unwrap();

	let annotation_type = captures.name("annotation");

	if annotation_type.is_none() {
		return None;
	}

	match annotation_type.unwrap().as_str() {
		"desc" => Some(parse_desc(line)),
		"param" => Some(parse_param(line)),
		"return" => Some(parse_return(line)),
		"method" => Some(Annotation::Method),
		_ => None,
	}
}

pub fn parse_comments(tokens: TokenStream) -> HashMap<String, Vec<Annotation>> {
	let comments = get_doc_groups(tokens.clone());

	let tree: HashMap<String, Vec<Annotation>> = comments
		.iter()
		.map(|(name, lines)| {
			let converted: Vec<Annotation> = lines
				.iter()
				.filter_map(|line| {
					let parsed = parse_comment_line(line);

					parsed
				})
				.collect();

			(name.to_string(), converted)
		})
		.collect();

	tree
}
