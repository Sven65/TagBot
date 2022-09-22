use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, parse_str, parse::Parse, Token, Attribute};

#[derive(Debug)]
enum AccessType {
	Function,
	Field,
}

#[derive(Debug)]
enum LuaType {
	StringOrNil,
}

#[derive(Debug)]
struct IndexInput {
	field: String,
	access_type: AccessType,
	lua_type: LuaType,
}

impl std::fmt::Display for IndexInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {:?}, {:?})", self.field, self.access_type, self.lua_type)
    }
}

impl Parse for IndexInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let attrs = input.call(Attribute::parse_inner)?;

		println!("Attrs {:#?}", attrs);

        let lookahead = input.lookahead1();
		println!("look {:?}", lookahead.peek(Token![::]));

		Ok(IndexInput {
			field: "".to_string(),
			access_type: AccessType::Field,
			lua_type: LuaType::StringOrNil,
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


#[proc_macro_attribute]
pub fn ud_index(attr: TokenStream, item: TokenStream) -> TokenStream {

	println!("attr: \"{}\"", attr.to_string());
	let parsed = parse_macro_input!(attr as IndexInput);

	println!("parsed {}", parsed);

    println!("item: \"{}\"", item.to_string());
    item

	// let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);
	// let output = quote! {
	// 	fn describe() {
	// 		println!("{} is {}.", stringify!(#ident), stringify!(attrs.get(0)));
	// 	}
	// };

	// output.into()
}