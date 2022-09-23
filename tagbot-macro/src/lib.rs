use proc_macro::{self, TokenStream};
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};


use index_input::IndexInput;

mod index_input;

#[proc_macro_attribute]
pub fn ud_index(args: TokenStream, item: TokenStream) -> TokenStream {
	let parsed_input = parse_macro_input!(args as IndexInput);

	let cloned_item = item.clone();

	let parsed_item = parse_macro_input!(cloned_item as syn::ItemImpl);

	// println!("parsed {}", parsed_input);

	println!("item {:#?}", parsed_item);
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

	// let case_string = format_ident!(&{}, parsed_input.field);
	let field = parsed_input.field;

	let case_tokens = quote! {
		&#field => {},
	};

	
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

	println!("final {:#?}", final_tokens.to_string());

	final_tokens.into()

    // item
}