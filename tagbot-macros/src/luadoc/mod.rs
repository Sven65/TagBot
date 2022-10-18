use std::{
	collections::{hash_map::Entry, HashMap},
	fs::File,
	io::Write,
};

use darling::ToTokens;
use proc_macro::TokenStream;
use syn::{
	parse_macro_input, Arm, AttributeArgs, Expr, ExprBlock, ExprMethodCall, Lit, NestedMeta, Path,
	Stmt,
};

use std::cell::RefCell;

use std::env;

use crate::luadoc::{
	convert_parser::parse_convert_type,
	document::{Document, Method},
	markdown::generate_markdown,
};

use self::{
	comments::parse_comments,
	convert_parser::parse_convert_dual_type,
	document::{Attribute, DocTitle},
	requireable::parse_requireable,
};

pub mod comments;
pub mod convert_parser;
pub mod document;
pub mod markdown;
pub mod requireable;

/// Parses the segments of a [`syn::Path`] into a string
///
/// # Arguments
///
/// * `path` - The [`syn::Path`] whose segments to parse
pub fn parse_path(path: &Path) -> String {
	path.segments
		.clone()
		.into_iter()
		.map(|s| s.ident.to_string())
		.reduce(|cur: String, nxt: String| cur + "::" + &nxt)
		.unwrap()
}

/// Parses the arguments to the macro into a vec of strings
///
/// # Arguments
///
/// * `input` - A vec of [`syn::NestedMeta`] to parse
fn parse_args(input: Vec<NestedMeta>) -> Vec<String> {
	let mut args: Vec<String> = Vec::new();

	input.iter().for_each(|arg| {
		let data: String = match arg {
			NestedMeta::Lit(lit) => match lit {
				Lit::Str(str) => str.value(),
				_ => panic!("Invalid argument {:#?} supplied", arg),
			},
			NestedMeta::Meta(meta) => match meta {
				syn::Meta::Path(path) => parse_path(path),
				syn::Meta::NameValue(name) => {
					let path = parse_path(&name.path);

					let value = match &name.lit {
						Lit::Str(str) => str.value(),
						_ => panic!("Value of attribute is not string."),
					};

					format!("{}={}", path, value)
				}
				_ => {
					panic!("Invalid non-path supplied to meta");
				}
			},
		};

		args.push(data);
	});

	args
}

/// Finds a meta method by name from a [`syn::Expr`] containing a [`syn::Expr::MethodCall`]
///
/// # Arguments
/// * `expr` - The [`syn::Expr`] to parse
/// * `method` - The rlua metamethod name to find
fn find_meta_method<'a>(expr: &Expr, method: &str) -> Option<ExprMethodCall> {
	match expr {
		Expr::MethodCall(call) => {
			if call.method.to_string() == "add_meta_method" {
				let mut method_call: Option<ExprMethodCall> = None;
				call.args.clone().into_iter().find(|arg| match arg {
					Expr::Path(path) => {
						let parsed_path = parse_path(&path.path);

						let is_found = parsed_path == method;

						if is_found {
							method_call = Some(call.clone());
						}

						is_found
					}
					_ => false,
				});

				method_call
			} else {
				return None;
			}
		}
		_ => {
			return None;
		}
	}
}

/// Parses the body of a MetaMethod::Index closure
///
/// # Arguments
/// * `body` - The [`syn::ExprBlock`] to parse
fn parse_index_body(body: &ExprBlock) -> Vec<Attribute> {
	let call = body.block.stmts.get(0).unwrap();

	let call = match call {
		Stmt::Expr(expr) => match expr {
			Expr::Call(call) => call,
			_ => panic!("Expr is not call"),
		},
		_ => panic!("Stmt is not expr"),
	};

	let match_arg = match call.args.first().unwrap() {
		Expr::Match(m) => m,
		_ => panic!("First arg is not match"),
	};

	let arms: Vec<Attribute> = match_arg
		.arms
		.iter()
		.map(parse_arm)
		.filter(|arm| arm.is_some())
		.map(|arm| arm.unwrap())
		.collect();

	arms
}

/// Parses the body of a single match arm
///
/// # Arguments
/// * `expr` - The [`syn::Expr`] to parse
fn parse_arm_body(expr: Expr) -> (String, bool) {
	let method = match expr.clone() {
		Expr::Try(tryexp) => match &*tryexp.expr {
			Expr::Call(call) => match &*call.func {
				Expr::Path(path) => parse_path(&path.path),
				_ => panic!("Call func is not path"),
			},
			_ => panic!("try expr is not call"),
		},
		_ => panic!("Expr is not call"),
	};

	let (typ, optional) = match method.as_str() {
		"convert_type" => parse_convert_type(expr.clone(), false),
		"convert_constructable" => parse_convert_type(expr.clone(), false),
		"convert_constructable2" => parse_convert_type(expr.clone(), false),
		"convert_constructable2_option" => parse_convert_type(expr.clone(), true),
		"convert_type_option" => parse_convert_type(expr.clone(), true),
		"convert_constructable_option" => parse_convert_type(expr.clone(), true),
		"convert_vec" => {
			let (typ, optional) = parse_convert_type(expr.clone(), false);

			(format!("array[{}]", typ), optional)
		}
		"lua_todo" => ("<!> Unknown (Not implemented) <!>".to_string(), false),
		"convert_hashmap_types_with_new" => {
			let (typ1, typ2, optional) = parse_convert_dual_type(expr.clone(), false);

			(format!("HashMap<{}, {}>", typ1, typ2), optional)
		}
		"convert_hashmap_types" => {
			let (typ1, typ2, optional) = parse_convert_dual_type(expr.clone(), false);

			(format!("HashMap<{}, {}>", typ1, typ2), optional)
		}
		"convert_vec_new" => {
			let (typ, optional) = parse_convert_type(expr.clone(), false);

			(format!("array[{}]", typ), optional)
		}
		&_ => panic!("Handling for converter in {:#?} is not implemented.", expr),
	};

	(typ, optional)
}

/// Parses a single match arm
///
/// # Arguments
/// * `arm` - The [`syn::Arm`] to parse
fn parse_arm(arm: &Arm) -> Option<Attribute> {
	let name = match &arm.pat {
		syn::Pat::Lit(lit) => match &*lit.expr {
			Expr::Lit(lit) => match &lit.lit {
				syn::Lit::Str(str) => Some(str.value()),
				_ => panic!("lit expr is not str"),
			},
			_ => panic!("lit is not expr"),
		},
		syn::Pat::Reference(_) => None,
		syn::Pat::Wild(_) => None,
		_ => panic!("arm pat is invalid. {:#?}", arm.pat),
	};

	if name.is_none() {
		return None;
	}

	let (typ, optional) = parse_arm_body(*arm.body.clone());

	let name = name.unwrap();

	return Some(Attribute { name, optional, typ });
}

/// Parses a `add_meta_method(MetaMethod::Index)` call
///
/// # Arguments
/// * `tokens` - The TokenStream from the macro to parse
fn parse_index_method(tokens: TokenStream) -> Vec<Attribute> {
	let ast: syn::ItemFn = syn::parse(tokens.clone()).unwrap();

	// let stmt = ast.block.stmts.get(0).unwrap();

	let mut expr: Option<ExprMethodCall> = None;

	ast.block.stmts.iter().find(|stmt| {
		let data = match stmt {
			syn::Stmt::Expr(expr) => find_meta_method(&expr, "MetaMethod::Index"),
			syn::Stmt::Semi(expr, _semi) => find_meta_method(&expr, "MetaMethod::Index"),
			_ => None,
		};

		expr = data.clone();

		data.is_some()
	});

	if expr.is_none() {
		panic!("Failed to find MetaMethod::Index adding");
	}

	let expr = expr.unwrap();

	let closure: Vec<Expr> = expr.args.into_iter().collect();

	let closure = closure.get(1).unwrap();

	let closure = match closure {
		Expr::Closure(c) => c,
		_ => panic!("Didn't find closure at expected position."),
	};

	let body = match &*closure.body {
		syn::Expr::Block(block) => parse_index_body(block),
		_ => panic!("Closure body has wrong type"),
	};

	body
}

/// Gets groups that have the type "doc" and returns a vec of strings
///
/// # Arguments
/// * `tokens` - The tokens to parse
pub fn get_doc_groups(tokens: TokenStream) -> Vec<String> {
	let ast: syn::ItemStruct = syn::parse(tokens.clone()).unwrap();

	let docs: Vec<String> = ast
		.attrs
		.into_iter()
		.filter(|attr| {
			let ppath = parse_path(&attr.path);

			ppath == "doc"
		})
		.map(|args| {
			args.tokens
				.into_iter()
				.map(|arg| match arg {
					proc_macro2::TokenTree::Literal(lit) => {
						let lit: syn::Lit = syn::parse2(lit.to_token_stream()).unwrap();

						Some(lit)
					}
					_ => None,
				})
				.filter(|arg| arg.is_some())
				.filter(|arg| match arg.as_ref().unwrap() {
					syn::Lit::Str(_) => true,
					_ => false,
				})
				.map(|arg| match arg.unwrap() {
					syn::Lit::Str(str) => Some(str),
					_ => None,
				})
				.filter(|arg| arg.is_some())
				.map(|arg| arg.unwrap().value())
				.collect()
		})
		.collect();

	docs
}

/// Parses out the title for a doc
fn get_doc_title(tokens: TokenStream) -> String {
	let ast: syn::ItemStruct = syn::parse(tokens.clone()).unwrap();

	ast.ident.to_string()
}

/// Generates a [`DocTitle`] for a doc
fn generate_class_doc(tokens: TokenStream) -> DocTitle {
	let docs = get_doc_groups(tokens.clone());

	let title = get_doc_title(tokens.clone());

	DocTitle { title, note: docs }
}

thread_local!(static CURRENT_DOC: RefCell<HashMap<String, Document>> = RefCell::new(HashMap::new()));

pub fn lua_doc_generator(args: TokenStream, tokens: TokenStream) -> TokenStream {
	let should_generate = match env::var("GENERATE_DOCS") {
		Ok(val) => val.to_lowercase() == "true",
		Err(_e) => false,
	};

	if !should_generate {
		return tokens;
	}

	let input: AttributeArgs = parse_macro_input!(args as AttributeArgs);

	let parsed_args = parse_args(input);

	let (class_name, parsed_second) = parsed_args.split_first().unwrap();

	let class_name = class_name.to_string();

	if parsed_args.contains(&"index".to_string()) {
		let parsed_index = parse_index_method(tokens.clone());

		CURRENT_DOC.with(|map| {
			let mut borrowed = map.borrow_mut();

			let doc = match borrowed.entry(class_name.clone()) {
				Entry::Occupied(o) => o.into_mut(),
				Entry::Vacant(v) => v.insert(Document::new()),
			};

			doc.attributes = parsed_index;
		});
	};

	let parsed_args = parsed_second;

	if parsed_args.contains(&"class".to_string()) {
		if parsed_args.len() > 1 {
			panic!("Error! `Class` cannot be used with other doc types.")
		}

		let title = generate_class_doc(tokens.clone());

		// CURRENT_DOC.title = title;
		CURRENT_DOC.with(|map| {
			let mut borrowed = map.borrow_mut();

			let doc = match borrowed.entry(class_name.clone()) {
				Entry::Occupied(o) => o.into_mut(),
				Entry::Vacant(v) => v.insert(Document::new()),
			};

			doc.title = title;
		});
	}

	if parsed_args.contains(&"parse_comments".to_string()) {
		let comment_tree = parse_comments(tokens.clone());

		let methods: HashMap<String, Method> = comment_tree
			.iter()
			.filter(|(_, b)| b.contains(&comments::Annotation::Method))
			.map(|(a, b)| {
				let method = Method::from(b);

				(a.to_string(), method)
			})
			.collect::<HashMap<String, Method>>();

		CURRENT_DOC.with(|map| {
			let mut borrowed = map.borrow_mut();

			let doc = match borrowed.entry(class_name.clone()) {
				Entry::Occupied(o) => o.into_mut(),
				Entry::Vacant(v) => v.insert(Document::new()),
			};

			doc.methods = methods.into();
		});
	}

	let contains_requireable = parsed_args.iter().any(|arg| {
		let split: Vec<&str> = arg.split("=").collect();

		if split.len() != 2 {
			return false;
		}

		let first = split.get(0).unwrap().to_string();
		let value = split.get(1).unwrap().to_string();

		let is_requireable = first == "requireable";

		if is_requireable {
			CURRENT_DOC.with(|map| {
				let mut borrowed = map.borrow_mut();

				let doc = match borrowed.entry(class_name.clone()) {
					Entry::Occupied(o) => o.into_mut(),
					Entry::Vacant(v) => v.insert(Document::new()),
				};

				doc.requireable_as = Some(value);
			});
		}

		is_requireable
	});

	if contains_requireable {
		let function_tree = parse_requireable(tokens.clone());

		let functions: HashMap<String, Method> = function_tree
			.iter()
			.filter(|(_, b)| b.contains(&comments::Annotation::Function))
			.map(|(a, b)| {
				let method = Method::from(b);

				(a.to_string(), method)
			})
			.collect::<HashMap<String, Method>>();

		CURRENT_DOC.with(|map| {
			let mut borrowed = map.borrow_mut();

			let doc = match borrowed.entry(class_name.clone()) {
				Entry::Occupied(o) => o.into_mut(),
				Entry::Vacant(v) => v.insert(Document::new()),
			};

			doc.requireable = true;
			doc.requireable_functions = functions.into();
		});
	}

	CURRENT_DOC.with(|doc| {
		let borrowed = doc.borrow();

		borrowed.iter().for_each(|(name, b)| {
			let generated_md = generate_markdown(b);

			let dir_path = "docs/lua";
			let path = format!("{}/{}.md", dir_path, name);
			std::fs::create_dir_all(dir_path.clone()).unwrap();

			let mut output = File::create(path).unwrap();

			write!(output, "{}", generated_md).unwrap();
		});
	});

	tokens
}
