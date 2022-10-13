use proc_macro::TokenStream;
use syn::{
	parse_macro_input, token::Semi, Arm, AttributeArgs, Block, Expr, ExprBlock, ExprCall,
	ExprClosure, ExprMethodCall, Lit, Local, NestedMeta, PatLit, Path, Stmt,
};

use crate::luadoc::convert_parser::parse_convert_type;

mod convert_parser;

#[derive(Debug)]
struct ParsedArm {
	name: String,
	typ: String,
	optional: bool,
}

pub fn parse_path(path: &Path) -> String {
	path.segments
		.clone()
		.into_iter()
		.map(|s| s.ident.to_string())
		.reduce(|cur: String, nxt: String| cur + "::" + &nxt)
		.unwrap()
}

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
				_ => panic!("Invalid non-path supplied to meta"),
			},
			_ => panic!("Invalid literal type supplied"),
		};

		args.push(data);
	});

	println!("finished parsing");

	args
}

fn find_meta_method<'a>(expr: &Expr, method: &str) -> Option<ExprMethodCall> {
	match expr {
		Expr::MethodCall(call) => {
			if call.method.to_string() == "add_meta_method" {
				let mut method_call: Option<ExprMethodCall> = None;
				let found = call.args.clone().into_iter().find(|arg| match arg {
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

fn parse_index_body(body: &ExprBlock) {
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

	let arms: Vec<ParsedArm> = match_arg
		.arms
		.iter()
		.map(parse_arm)
		.filter(|arm| arm.is_some())
		.map(|arm| arm.unwrap())
		.collect();

	println!("Arms {:#?}", arms);
}

fn parse_arm_body(expr: Expr) -> String {
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

	println!("path is {:#?}", method);

	let typ = match method.as_str() {
		"convert_type" => parse_convert_type(expr.clone()),
		&_ => "".to_string(),
	};

	typ
}

fn parse_arm(arm: &Arm) -> Option<ParsedArm> {
	let name = match &arm.pat {
		syn::Pat::Lit(lit) => match &*lit.expr {
			Expr::Lit(lit) => match &lit.lit {
				syn::Lit::Str(str) => Some(str.value()),
				_ => panic!("lit expr is not str"),
			},
			_ => panic!("lit is not expr"),
		},
		syn::Pat::Reference(_) => None,
		_ => panic!("arm pat is invalid"),
	};

	if name.is_none() {
		return None;
	}

	let typ = parse_arm_body(*arm.body.clone());

	let name = name.unwrap();

	return Some(ParsedArm { name, optional: false, typ });
}

fn parse_index_method(tokens: TokenStream) {
	let ast: syn::ItemFn = syn::parse(tokens.clone()).unwrap();

	// let stmt = ast.block.stmts.get(0).unwrap();

	let mut expr: Option<ExprMethodCall> = None;

	let stmt = ast.block.stmts.iter().find(|stmt| {
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
}

pub fn lua_doc_generator(args: TokenStream, tokens: TokenStream) -> TokenStream {
	// println!("tokens {:#?}", tokens);
	// println!("args {:#?}", args);

	let input: AttributeArgs = parse_macro_input!(args as AttributeArgs);

	let parsed_args = parse_args(input);

	if parsed_args.contains(&"index".to_string()) {
		parse_index_method(tokens.clone())
	}

	println!("parsed {:#?}", parsed_args);

	tokens
}
