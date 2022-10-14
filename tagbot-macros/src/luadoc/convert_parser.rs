use syn::{AngleBracketedGenericArguments, Expr, Path, PathArguments};

use crate::luadoc::parse_path;

fn parse_path_args(path: &Path) -> Vec<syn::PathArguments> {
	path.segments
		.clone()
		.into_iter()
		.map(|s| s.arguments)
		.collect()
}

pub fn get_arguments(expr: Expr) -> Vec<syn::PathArguments> {
	match expr {
		Expr::Try(tryexp) => match &*tryexp.expr {
			Expr::Call(call) => match &*call.func {
				Expr::Path(path) => {
					return parse_path_args(&path.path);
				}
				_ => panic!("Call func is not path"),
			},
			_ => panic!("try expr is not call"),
		},
		_ => panic!("Expr is not call"),
	};
}

pub fn parse_convert_type(expr: Expr, optional: bool) -> (String, bool) {
	let args = get_arguments(expr);

	let typ = match args.get(0).unwrap() {
		syn::PathArguments::AngleBracketed(args) => match args {
			AngleBracketedGenericArguments { args, .. } => {
				let first = args.first().unwrap();

				match first {
					syn::GenericArgument::Type(ty) => match ty {
						syn::Type::Path(path) => parse_path(&path.path),
						_ => panic!("Type is not path"),
					},
					_ => panic!("Argument is not type"),
				}
			}
		},
		_ => panic!("Args is not angle"),
	};

	return (typ, optional);
}
