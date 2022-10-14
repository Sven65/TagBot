use std::collections::HashMap;

use darling::ToTokens;
use proc_macro::TokenStream;

use crate::luadoc::parse_path;

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

pub fn parse_comments(tokens: TokenStream) {
	let comments = get_doc_groups(tokens.clone());

	println!("comments {:#?}", comments);
}
