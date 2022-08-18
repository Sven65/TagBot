use proc_macro::TokenStream;
use syn::{parse_macro_input, Lit};
use quote::quote;
use syn::spanned::Spanned;
// use syn::parse::{Error};

pub(crate) mod attributes;
pub(crate) mod structures;

#[macro_use]
pub(crate) mod util;

use attributes::*;
use structures::*;
use util::*;

#[proc_macro_attribute]
pub fn register_command(attr: TokenStream, input: TokenStream) -> TokenStream {
	println!("Attr {}", attr);
	println!("Input {}", input);

	let fun = parse_macro_input!(input as CommandFun);

	let _name = if attr.is_empty() {
		fun.name.to_string()
	} else {
		parse_macro_input!(attr as Lit).to_str()
	};

	let mut options = Options::new();

	for attribute in &fun.attributes {
		let _span = attribute.span();
		let values = propagate_err!(parse_values(attribute));

		let name = values.name.to_string();
		let name = &name[..];

		match name {
			"description" => {
				let line: String = propagate_err!(attributes::parse(values));
				options.description = line;
			},
			_ => {}
		}
	}

	println!("Parsed {:?}", fun);
	
	println!("Registering command {}", fun.name);

	let name = fun.name.clone();
	let _body = fun.body;

	let n = name.with_suffix("COMMAND");

	let Options {
		description,
	} = options;

	let options = name.with_suffix("_OPTIONS");

	let options_path = quote!(tagbot::);
	let command_path = quote!(serenity::framework::standard::Command);

	(quote! {
		pub static #options: #options_path = #options_path {
			description: #description,
		};

		pub static #n: #command_path = #command_path {
			fun: #name
		};
	}).into()

	// println!("Registering desc {}", fun.)

	// // Build the output, possibly using quasi-quotation
    // let expanded = quote! {
    //     // ...
    // };

    // // Hand the output tokens back to the compiler
    // return TokenStream::from(expanded);

	// return input
}


// #[proc_macro_attribute]
// pub fn description(attr: TokenStream, input: TokenStream) -> TokenStream {
// 	println!("desc Attr {}", attr);
// 	println!("desc Input {}", input);

// 	return attr
// }