use std::collections::HashMap;

use darling::ToTokens;
use proc_macro::TokenStream;
use syn::{Attribute, ExprMethodCall, ImplItemMethod};

use crate::luadoc::{
	comments::{parse_comment_line, Annotation},
	parse_path,
};

#[derive(Debug)]
struct ValueSetParams {
	pub index: String,
	pub func: String,
}

/// Parses a Vec of attributes to a Vec of strings
///
/// # Arguments
/// * `attribs` - The attributes to parse
fn parse_attributes_to_doc(attribs: Vec<Attribute>) -> Vec<String> {
	attribs
		.iter()
		.filter_map(|attr| {
			let ident = attr.path.get_ident().unwrap().to_string();

			if ident != "doc" {
				return None;
			}

			Some(attr)
		})
		.map(|attr| {
			attr.tokens
				.clone()
				.into_iter()
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
				.collect()
		})
		.collect()
}

/// Parses the ident name of the final return call of the `create_module` function
///
/// # Arguments
/// * `method` - The `create_module` method to parse
fn get_return_table_ident_name(method: &ImplItemMethod) -> String {
	let last_stmt = method.block.stmts.last().unwrap();

	let call = match last_stmt {
		syn::Stmt::Expr(expr) => match expr {
			syn::Expr::Call(call) => Some(call),
			_ => None,
		},
		_ => None,
	};

	if call.is_none() {
		panic!("Return value for requireable is not call");
	}

	let call = call.unwrap();

	let path = match &*call.func {
		syn::Expr::Path(path) => Some(parse_path(&path.path)),
		_ => None,
	};

	if path.is_none() {
		panic!("Failed to parse path for requireable return type");
	}

	let path = path.unwrap();

	if path != "Value::Table" {
		panic!("Expected Value::Table return from function, but none was found.");
	}

	let args: Vec<String> = call
		.args
		.iter()
		.filter_map(|arg| match arg {
			syn::Expr::MethodCall(method) => match &*method.receiver {
				syn::Expr::Path(path) => Some(parse_path(&path.path)),
				_ => None,
			},
			syn::Expr::Path(path) => Some(parse_path(&path.path)),
			_ => None,
		})
		.collect();

	let ident_name = args.get(0).unwrap();

	ident_name.to_string()
}

/// Parses `.set` calls from the final return call of the `create_module` function
///
/// # Arguments
/// * `call` - The `.set` call to parse
fn parse_table_set_args(call: &ExprMethodCall) -> ValueSetParams {
	let receiver = match &*call.receiver {
		syn::Expr::MethodCall(mcall) => Some(mcall),
		_ => None,
	};

	if receiver.is_none() {
		panic!("Method call does not have receiver.");
	}

	let receiver = receiver.unwrap();

	let args: Vec<String> = receiver
		.args
		.iter()
		.filter_map(|expr| match expr {
			syn::Expr::Lit(lit) => match &lit.lit {
				syn::Lit::Str(str) => Some(str.value()),
				_ => None,
			},
			syn::Expr::MethodCall(m_call) => match &*m_call.receiver {
				syn::Expr::Path(path) => Some(parse_path(&path.path)),
				_ => None,
			},
			_ => None,
		})
		.collect();

	ValueSetParams {
		index: args.get(0).unwrap().to_string(),
		func: args.get(1).unwrap().to_string(),
	}
}

/// Parses `.set` calls from the final return call of the `create_module` function
///
/// # Arguments
/// * `method` - The `create_module` method to parse
/// * `ident_name` - The ident being returned by the function
fn parse_return_sets(method: &ImplItemMethod, ident_name: String) -> Vec<ValueSetParams> {
	let added_values: Vec<&ExprMethodCall> = method
		.block
		.stmts
		.iter()
		.filter_map(|stmt| {
			let call = match stmt {
				syn::Stmt::Expr(expr) => match expr {
					syn::Expr::MethodCall(call) => Some(call),
					_ => None,
				},
				syn::Stmt::Semi(expr, _) => match expr {
					syn::Expr::MethodCall(call) => Some(call),
					_ => None,
				},
				_ => None,
			};

			call
		})
		.filter_map(|call| match &*call.receiver {
			syn::Expr::MethodCall(method_call) => match &*method_call.receiver {
				syn::Expr::Path(path) => {
					let parsed_path = parse_path(&path.path);

					if parsed_path != ident_name {
						return None;
					}

					if method_call.method.to_string() != "set" {
						return None;
					}

					Some(call)
				}
				_ => None,
			},
			_ => None,
		})
		.collect();

	let table_setters: Vec<ValueSetParams> = added_values
		.iter()
		.map(|call| parse_table_set_args(call.to_owned()))
		.collect();

	table_setters
}

/// Finds any function creators
///
/// This means any `ctx.create_function` calls.
///
/// # Arguments
/// * `method` - The `create_module` function to parse
fn find_function_creators(method: &ImplItemMethod) -> Vec<&syn::Local> {
	let found_methods: Vec<&syn::Local> = method
		.block
		.stmts
		.iter()
		.filter_map(|stmt| match stmt {
			syn::Stmt::Local(local) => {
				let init = local.init.as_ref().unwrap();
				match &*init.1 {
					syn::Expr::MethodCall(call) => {
						let called_method = call.method.to_string();

						if called_method == "create_function" {
							Some(local)
						} else {
							None
						}
					}
					_ => None,
				}
			}
			_ => None,
		})
		.collect();

	found_methods
}

/// Finds docstring comments for a Vec of [`syn::Local`]
///
/// # Arguments
/// * `locals` - The locals to parse
/// * `methods` - The methods to find docstrings for
fn find_function_creator_docs(
	locals: Vec<&syn::Local>,
	methods: Vec<ValueSetParams>,
) -> HashMap<String, Vec<String>> {
	let parsed: HashMap<String, Vec<String>> = locals
		.iter()
		.filter_map(|local| {
			let ident = match &local.pat {
				syn::Pat::Ident(ident) => Some(ident.ident.to_string()),
				_ => None,
			};

			if ident.is_none() {
				return None;
			}

			Some((ident.unwrap(), local))
		})
		.filter(|(name, _)| methods.iter().any(|method| method.func == name.to_string()))
		.map(|(name, local)| {
			let docs = parse_attributes_to_doc(local.attrs.clone());

			let method: Vec<&ValueSetParams> = methods
				.iter()
				.filter(|m| m.func == name.to_string())
				.collect();

			let method = method.get(0).unwrap();

			(method.index.to_string(), docs)
		})
		.collect();

	parsed
}

/// Parses function docs
///
/// # Arguments
/// * `docs` - The docs to parse
fn parse_function_docs(docs: HashMap<String, Vec<String>>) -> HashMap<String, Vec<Annotation>> {
	docs.iter()
		.map(|(name, doc)| {
			let parsed_lines: Vec<Annotation> = doc.iter().filter_map(parse_comment_line).collect();

			(name.to_string(), parsed_lines)
		})
		.collect::<HashMap<String, Vec<Annotation>>>()
}

/// Parses a method with a table return value
///
/// # Arguments
/// * `method` - The method to parse
fn parse_return_table(method: &ImplItemMethod) -> HashMap<String, Vec<Annotation>> {
	let return_ident_name = get_return_table_ident_name(method);

	let return_sets = parse_return_sets(method, return_ident_name);

	let creators = find_function_creators(method);

	let found_docs = find_function_creator_docs(creators, return_sets);

	let docs = parse_function_docs(found_docs);

	docs
}

/// Parses a requireable trait implementation
///
/// # Arguments
/// * `tokens` - The tokens to parse
pub fn parse_requireable(tokens: TokenStream) -> HashMap<String, Vec<Annotation>> {
	let ast: syn::ItemImpl = syn::parse(tokens.clone()).unwrap();

	let found_methods: Vec<&ImplItemMethod> = ast
		.items
		.iter()
		.filter_map(|item| match item {
			syn::ImplItem::Method(method) => {
				let method_name = method.sig.ident.to_string();

				if method_name != "create_module".to_string() {
					return None;
				}

				Some(method)
			}
			_ => None,
		})
		.collect();

	let method = found_methods.get(0).unwrap().to_owned();

	let doc_comments = parse_attributes_to_doc(method.attrs.clone());
	let parsed_comments: Vec<Annotation> = doc_comments
		.iter()
		.filter_map(parse_comment_line)
		.collect::<Vec<Annotation>>();

	let return_type: Vec<String> = parsed_comments
		.iter()
		.filter_map(|annotation| match annotation {
			Annotation::Return(ret) => Some(ret.typ.clone()),
			_ => None,
		})
		.collect();

	let return_type = return_type.get(0).unwrap();

	return match return_type.as_str() {
		"table" => parse_return_table(method),
		"module" => parse_return_table(method),
		_ => panic!(
			"Handling of return type '{}' for requireable is not implemented.",
			return_type
		),
	};
}
