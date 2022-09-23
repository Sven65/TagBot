use std::str::FromStr;

use strum_macros::EnumString;
use syn::{parse::Parse, Error};

#[derive(Debug, EnumString)]
pub enum AccessType {
	Function,
	Field,
}

#[derive(Debug, EnumString)]
pub enum LuaType {
	StringOrNil,
	Value,
}

#[derive(Debug)]
pub struct IndexInput {
	pub field: String,
	pub access_type: AccessType,
	pub lua_type: LuaType,
	pub accessor_field: String,
}

impl std::fmt::Display for IndexInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(field = {}, access_type = {:?}, lua_type = {:?}, accessor_field = {})", self.field, self.access_type, self.lua_type, self.accessor_field)
    }
}

impl IndexInput {}

impl Parse for IndexInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, Error> {
		let attrs = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)
		.unwrap();

		let field: syn::Result<String> = match &attrs[0] {
			syn::Expr::Lit(pat) => {
				let token = match &pat.lit {
					syn::Lit::Str(p) => p.token().to_string().replace("\"", ""),
					_ => panic!("Invalid token type found for field")
				};

				Ok(token)
			},
			_ => panic!("Invalid literal type for field name. Received: {:#?}", &attrs[0])
		};

		let access_type: syn::Result<AccessType> = match &attrs[1] {
			syn::Expr::Path(pat) => {
				let path_strings: Vec<String> = (&pat.path.segments).into_iter().map(|segment| {
					segment.ident.to_string()
				}).collect();

				let access_type = AccessType::from_str(path_strings.last().unwrap());

				if access_type.is_err() {
					panic!("Failed to parse access type into enum: {}", access_type.unwrap_err());
				}

				Ok(access_type.unwrap())
			},
			_ => panic!("Invalid token type for access type.")
		};

		let accessor_field: syn::Result<String> = match &attrs[2] {
			syn::Expr::Lit(pat) => {
				let token = match &pat.lit {
					syn::Lit::Str(p) => p.token().to_string().replace("\"", ""),
					_ => panic!("Invalid token type found for accessor field")
				};

				Ok(token)
			},
			_ => panic!("Invalid literal type for accessor field. Received: {:#?}", &attrs[2])
		};

		let lua_type: syn::Result<LuaType> = match &attrs[3] {
			syn::Expr::Path(pat) => {
				let path_strings: Vec<String> = (&pat.path.segments).into_iter().map(|segment| {
					segment.ident.to_string()
				}).collect();

				let lua_type = LuaType::from_str(path_strings.last().unwrap());

				if lua_type.is_err() {
					panic!("Failed to parse lua type into enum: {}", lua_type.unwrap_err());
				}

				Ok(lua_type.unwrap())
			},
			_ => panic!("Invalid token type for lua type.")
		};

		Ok(IndexInput {
			field: field.unwrap(),
			access_type: access_type.unwrap(),
			lua_type: lua_type.unwrap(),
			accessor_field: accessor_field.unwrap(),
		})
    }
}

