use crate::index_input::{IndexInput, AccessType};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident, ToTokens};

pub fn string_or_nil(input: &IndexInput) -> TokenStream2 {
	let accessor_field = &input.accessor_field;

	let ident = format!("this.0.{}", accessor_field);

	let expr: syn::Expr = syn::parse_str(&ident).expect("Unable to parse");

	let value_getter: TokenStream2 = match input.access_type {
		AccessType::Function => quote! { let gotten_value = #expr(); },
		AccessType::Field => quote! { let gotten_value = #expr; },
	};

	quote! {
		#value_getter

		if gotten_value.is_none() {
			Value::Nil
		} else {
			gotten_value.unwrap().to_lua(ctx)?
		}
	}
}

pub fn value(input: &IndexInput) -> TokenStream2 {
	let accessor_field = &input.accessor_field;

	let ident = format!("this.0.{}", accessor_field);

	let expr: syn::Expr = syn::parse_str(&ident).expect("Unable to parse");

	let value_getter: TokenStream2 = match input.access_type {
		AccessType::Function => quote! { let gotten_value = #expr(); },
		AccessType::Field => quote! { let gotten_value = #expr; },
	};

	quote! {
		#value_getter

		gotten_value.copy().to_lua(ctx)?
	}
}

pub fn convert(input: &IndexInput) -> TokenStream2 {
	let accessor_field = &input.accessor_field;
	let converter_type = &input.convert_to;

	if converter_type.is_none() {
		panic!("Tried to convert with unspecified conversion type.");
	}

	let ident = format!("this.0.{}", accessor_field);
	// let convert_ident = format!("{}::new(gotten_value)", converter_type.to);

	let expr: syn::Expr = syn::parse_str(&ident).expect("Unable to parse");
	let converter_expr: syn::Expr = syn::parse_str(&ident).expect("Unable to parse");

	let value_getter: TokenStream2 = match input.access_type {
		AccessType::Function => quote! { let gotten_value = #expr(); },
		AccessType::Field => quote! { let gotten_value = #expr; },
	};

	quote! {
		#value_getter

		let converted_value = #converter_type::new(gotten_value.unwrap());

		converted_value.to_lua(ctx)?
	}
}