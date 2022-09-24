use functions::string_or_nil;
use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use syn::{parse_macro_input};


use index_input::IndexInput;

use crate::{index_input::{AccessType, LuaType}, functions::{value, convert, string, convert_or_nil}};
mod index_input;
mod functions;

fn render_fields(fields: Vec::<IndexInput>) -> TokenStream2 {
	let mut streams: Vec::<TokenStream2> = Vec::new();

	for (_, field) in fields.iter().enumerate() {
		let field_name = &field.field;

		let value_func = match &field.lua_type {
			LuaType::StringOrNil =>  string_or_nil(field),
			LuaType::Value => value(field),
			LuaType::Convert => convert(field),
			LuaType::String => string(field),
			LuaType::ConvertOrNil => convert_or_nil(field),
		};

		println!("value func {:#?}", value_func.to_string());

		let stream = quote! {
			&#field_name => {#value_func},
		};

		streams.push(stream);
	}

	streams.push(quote! {
		&_ => Value::Nil
	});

	let case_tokens = quote! {
		#(#streams)*,
	};

	case_tokens
}

#[proc_macro_attribute]
pub fn ud_index(args: TokenStream, item: TokenStream) -> TokenStream {
	let cloned_item = item.clone();
	let parsed_item = parse_macro_input!(cloned_item as syn::ItemImpl);

	let parsed_input = parse_macro_input!(args as IndexInput);

	println!("parsed inp {:#?}", parsed_input);

	let struct_name: syn::Result<String> = match *parsed_item.self_ty {
		syn::Type::Path(path) => {
			let path_strings: Vec<String> = (path.path.segments).into_iter().map(|segment| {
				segment.ident.to_string()
			}).collect();

			let final_string = path_strings.join("::");

			Ok(final_string)
		},
		_=> panic!("Failed to get struct name for macro"),
	};

	let struct_name = struct_name.unwrap();

	let mut fields: Vec::<IndexInput> = Vec::new();
	
	fields.push(parsed_input);

	for (_idx, attr) in parsed_item.attrs.iter().enumerate() {
		let attr_args = attr.parse_args::<IndexInput>();

		if attr_args.is_err() {
			panic!("Failed to parse attribute arguments for macro.");
		}
		
		fields.push(attr_args.unwrap());
	}

	let case_tokens = render_fields(fields);

	let struct_ident = format_ident!("{}", struct_name);

	let final_tokens = quote! {
		impl UserData for #struct_ident {
			fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
				methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
					Ok(match &value.as_str() {
						#case_tokens
					})
				})
			}
		}
	};

	final_tokens.into()

    // item
}