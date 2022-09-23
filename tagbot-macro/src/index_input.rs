use std::str::FromStr;

use strum_macros::EnumString;
use syn::{parse::Parse, punctuated::Punctuated, Expr, Error, Token};

#[derive(Debug, EnumString)]
enum AccessType {
	Function,
	Field,
}

#[derive(Debug, EnumString)]
enum LuaType {
	StringOrNil,
}

#[derive(Debug)]
pub struct IndexInput {
	field: String,
	access_type: AccessType,
	lua_type: LuaType,
}

impl std::fmt::Display for IndexInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {:?}, {:?})", self.field, self.access_type, self.lua_type)
    }
}

impl IndexInput {}

impl Parse for IndexInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, Error> {
		let attrs = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)
		.unwrap();

		// let attrs = Punctuated::parse_separated_nonempty_with(input, Expr::parse)?;

		println!("attrs {:#?}", attrs);

		let field: syn::Result<String> = match &attrs[0] {
			syn::Expr::Lit(pat) => {
				let token = match &pat.lit {
					syn::Lit::Str(p) => p.token().to_string(),
					_ => panic!("Invalid token type found for field")
				};

				Ok(token)
			},
			_ => panic!("Invalid literal type for field")
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

		let lua_type: syn::Result<LuaType> = match &attrs[2] {
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

		// let access_type = &attrs[1];
		// let lua_type = &attrs[2];


		// println!("old field {:#?}", &attrs[0]);

		println!("field {:#?}", field);
		println!("type {:#?}", access_type);
		println!("lua type {:#?}", lua_type);


		Ok(IndexInput {
			field: field.unwrap(),
			access_type: access_type.unwrap(),
			lua_type: lua_type.unwrap(),
		})
        // if lookahead.peek(Token![struct]) {
        //     input.parse().map(Item::Struct)
        // } else if lookahead.peek(Token![enum]) {
        //     input.parse().map(Item::Enum)
        // } else {
        //     Err(lookahead.error())
        // }
    }
}

