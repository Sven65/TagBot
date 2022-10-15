use syn::{AngleBracketedGenericArguments, Expr, Path, TypeReference};

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

fn parse_type_reference(t_ref: &TypeReference) -> String {
	let p_path = match &*t_ref.elem {
		syn::Type::Path(pat) => parse_path(&pat.path),
		_ => panic!("t_ref elem is not path"),
	};

	format!("&{}", p_path)
}

pub fn parse_convert_type(expr: Expr, optional: bool) -> (String, bool) {
	let args = get_arguments(expr.clone());

	let typ = match args.get(0).unwrap() {
		syn::PathArguments::AngleBracketed(args) => match args {
			AngleBracketedGenericArguments { args, .. } => {
				let first = args.first().unwrap();

				match first {
					syn::GenericArgument::Type(ty) => match ty {
						syn::Type::Path(path) => parse_path(&path.path),
						syn::Type::Reference(t_ref) => parse_type_reference(t_ref),
						_ => panic!("Type is not path. {:#?}", ty),
					},
					_ => panic!("Argument is not type"),
				}
			}
		},
		syn::PathArguments::None => panic!("Failed to find type for index."),
		_ => panic!("Args is not angle."),
	};

	return (typ, optional);
}
