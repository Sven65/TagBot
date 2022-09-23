use proc_macro::{self, TokenStream};
use syn::{parse_macro_input};


use index_input::IndexInput;

mod index_input;

#[proc_macro_attribute]
pub fn ud_index(args: TokenStream, item: TokenStream) -> TokenStream {
	let parsed = parse_macro_input!(args as IndexInput);

	println!("parsed {}", parsed);

    item
}