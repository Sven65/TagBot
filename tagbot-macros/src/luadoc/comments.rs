use std::{collections::HashMap, hash::Hash, ops::Index};

use darling::ToTokens;
use indexmap::IndexMap;
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
pub struct TableAnnotation {
	/// The type of annotation (@[type])
	pub annotation_type: String,
	/// The parsed type ({type})
	pub typ: String,
	/// The description of the string
	pub desc: String,
	/// The param being documented
	pub param: String,

	pub children: Vec<Annotation>,
}

/// Annotation
#[derive(Debug, PartialEq, Clone)]
pub enum Annotation {
	Function,
	Method,
	Param(ParamValueAnnotation),
	Return(ParamAnnotation),
	ReturnParam(ParamValueAnnotation),
	ReturnTable(TableAnnotation),
	Description(String),
	Table(TableAnnotation),
	None,
}

impl Annotation {
	pub fn typ(&self) -> Option<String> {
		match self {
			Annotation::Function => None,
			Annotation::Method => None,
			Annotation::Param(param) => Some(param.typ.clone()),
			Annotation::ReturnParam(param) => Some(param.typ.clone()),
			Annotation::Return(ret) => Some(ret.typ.clone()),
			Annotation::Description(_) => None,
			Annotation::Table(tbl) => Some(tbl.typ.clone()),
			Annotation::ReturnTable(tbl) => Some(tbl.typ.clone()),
			Annotation::None => None,
		}
	}

	pub fn variant_string(&self) -> &str {
		match self {
			Annotation::Function => "function",
			Annotation::Method => "method",
			Annotation::Param(_) => "param",
			Annotation::Return(_) => "return",
			Annotation::ReturnParam(_) => "returnparam",
			Annotation::Description(_) => "desc",
			Annotation::Table(_) => "table",
			Annotation::ReturnTable(_) => "returntable",
			Annotation::None => "none",
		}
	}
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
	let parsed_value = parse_param_value_annotation(line);

	if parsed.is_none() || parsed_value.is_none() {
		panic!("Return parser could not execute.");
	}

	let parsed = parsed.unwrap();

	if parsed.typ == "table" {
		return Annotation::ReturnParam(parsed_value.unwrap());
	}

	if parsed_value.is_some() {
		let uw_parsed_value = parsed_value.unwrap();

		let param_parts = uw_parsed_value.param.split(".").collect::<Vec<&str>>();

		if param_parts.len() > 1 {
			return Annotation::ReturnParam(uw_parsed_value);
		}
	}

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

fn find_tables(tree: HashMap<String, Vec<Annotation>>) -> IndexMap<String, Vec<Annotation>> {
	let parsed_tables: IndexMap<String, Vec<Annotation>> = tree
		.iter()
		.filter_map(|(name, annots)| {
			let mut table_annots: IndexMap<String, Annotation> = IndexMap::new();
			let mut table_params: IndexMap<String, Vec<Annotation>> = IndexMap::new();

			annots
				.iter()
				.filter(|annot| {
					// Filter so we only have param and return variants
					let variant = annot.variant_string();

					variant == "param" || variant == "returnparam"
				})
				.filter_map(|annot| {
					// Filter out annots without typ()

					if annot.typ().is_some() {
						Some(annot)
					} else {
						None
					}
				})
				.for_each(|annot| {
					// Find param that's marked as table

					// todo: deduplicate code

					match annot {
						Annotation::Param(param) => {
							if param.typ == "table" {
								table_params.insert(param.param.clone(), Vec::new());
								table_annots.insert(param.param.clone(), annot.clone());
							} else {
								// Get table key
								// todo: Make this support multi-key tables (like.this.key.you.know)

								let parts = param.param.split(".").collect::<Vec<&str>>();

								if parts.len() > 1 {
									let key = parts.get(0).unwrap().to_string();
									let index = parts.get(1).unwrap();

									let table = table_params.get_mut(&key);

									if table.is_none() {
										panic!("table 1 is none!!!!!1");
									}

									let table = table.unwrap();

									// table.push(annot.to_owned());
									table.push(annot.to_owned());
								}
							}
						}
						Annotation::ReturnParam(param) => {
							if param.typ == "table" {
								table_params.insert(param.param.clone(), Vec::new());
								table_annots.insert(param.param.clone(), annot.clone());
							} else {
								// Get table key
								// todo: Make this support multi-key tables (like.this.key.you.know)

								let parts = param.param.split(".").collect::<Vec<&str>>();

								if parts.len() > 1 {
									let key = parts.get(0).unwrap().to_string();
									let index = parts.get(1).unwrap();

									let table = table_params.get_mut(&key);

									let table = table.unwrap();

									// table.push(annot.to_owned());
									table.push(annot.to_owned());
								}
							}
						}
						_ => panic!("Found unexpected annotation type when finding tables."),
					}
				});

			let params: Vec<Annotation> = table_params
				.iter()
				.map(|(param, annots)| {
					let table_annot = table_annots.get(param).unwrap();

					let mut table_type = "regular";

					let (annotation_type, desc, param_param, typ) = match table_annot {
						Annotation::Param(param) => (
							param.annotation_type.clone(),
							param.desc.clone(),
							param.param.clone(),
							param.typ.clone(),
						),
						Annotation::ReturnParam(param) => {
							table_type = "return";
							(
								param.annotation_type.clone(),
								param.desc.clone(),
								param.param.clone(),
								param.typ.clone(),
							)
						}
						_ => panic!(
							"Handling not implemented for table annot {:#?}.",
							table_annot
						),
					};

					if table_type == "return" {
						Annotation::ReturnTable(TableAnnotation {
							annotation_type,
							children: annots.to_vec(),
							desc,
							param: param_param,
							typ,
						})
					} else {
						Annotation::Table(TableAnnotation {
							annotation_type,
							children: annots.to_vec(),
							desc,
							param: param_param,
							typ,
						})
					}
				})
				.collect::<Vec<Annotation>>();

			Some((name.to_string(), params))
		})
		.collect::<IndexMap<String, Vec<Annotation>>>();

	parsed_tables
}

fn replace_params_with_table(
	original: HashMap<String, Vec<Annotation>>,
	tables: IndexMap<String, Vec<Annotation>>,
) {
	let mut new_tree: HashMap<String, Vec<Annotation>> = original.clone();

	tables
		.iter()
		.filter(|(_, annots)| annots.len() > 0)
		.for_each(|(name, annots)| {
			let group = new_tree.get_mut(name).unwrap();

			let new_group: Vec<Annotation> = group
				.iter()
				.filter_map(|annot| match annot {
					Annotation::Param(param) | Annotation::ReturnParam(param) => {
						if param.typ == "table" {
							let table_annot: Option<TableAnnotation> =
								annots.iter().find_map(|annot| match annot {
									Annotation::Table(table) | Annotation::ReturnTable(table) => {
										println!("table is {:#?}, param is {:#?}", table, param);
										if table.param == param.param {
											Some(table.to_owned())
										} else {
											println!("returning None for table param match");
											None
										}
									}
									_ => panic!("Found something other than table annotation."),
								});

							if table_annot.is_none() {
								panic!("Table annot is None.");
							}

							let table_annot = table_annot.unwrap();

							if param.param == table_annot.param {
								return Some(Annotation::Table(table_annot));
							} else {
								return Some(annot.to_owned());
							}
						} else {
							match annot {
								Annotation::Param(param) => {
									let parts = param.param.split(".").collect::<Vec<&str>>();

									if parts.len() < 2 {
										return Some(annot.to_owned());
									}

									None
								}
								_ => Some(annot.to_owned()),
							}
						}
					}
					Annotation::Return(ret) => None,
					_ => Some(annot.to_owned()),
				})
				.collect::<Vec<Annotation>>();

			println!("new group is {:#?}", new_group);

			new_tree.insert(name.to_string(), new_group);
		});

	println!("new tree is {:#?}", new_tree);
}

/// Parsed a comment string line into annotation
///
/// # Arguments
/// * `line` - The line to parse
pub fn parse_comment_line(line: &String) -> Option<Annotation> {
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
		"function" => Some(Annotation::Function),
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

	let tables = find_tables(tree.clone());

	replace_params_with_table(tree.clone(), tables);

	tree
}
