use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{self, parse_macro_input, DataStruct, Lit, LitStr};

/// A trait that allows for a wrapped enum to be stringified
/// with `tostring(...)` in a lua script.
///
/// ## Derivable
///
/// This trait is used with `#[derive]`, which adds the `[ConstructableFrom]` trait for constructing
/// and the `[rlua::UserData]` trait for `tostring`.
///
/// ```
/// // `derive` implements LuaEnum for TBWrapper.
/// #[derive(LuaEnum)]
/// pub struct TBWrapper(pub WrappedEnum);
/// ```
///
/// [impls]: #implementors
#[proc_macro_derive(LuaEnum)]
pub fn lua_enum(tokens: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

	let name = ast.ident;

	let data_struct: DataStruct = match ast.data {
		syn::Data::Struct(data) => data,
		_ => panic!("Failed to parse struct for LuaEnum Macro"),
	};

	let mut tuple_fields: Vec<&Ident> = Vec::new();

	for field in data_struct.fields.iter() {
		let ident: syn::Result<&Ident> = match &field.ty {
			syn::Type::Path(path) => {
				let ident = path.path.get_ident().unwrap();

				Ok(ident)
			}
			_ => panic!("Failed to parse path and ident for struct member."),
		};

		let ident_uw = ident.unwrap();

		tuple_fields.push(ident_uw);
	}

	let wrapped_ident = tuple_fields.get(0).unwrap();

	let gen = quote! {
		impl ConstructableFrom<#wrapped_ident> for #name {
			/// Creates a new wrapper
			///
			/// # Arguments
			/// * `value` - The #wrapped_ident to wrap
			fn new(value: #wrapped_ident) -> #name {
				#name(value)
			}
		}

		impl UserData for #name {
			fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
				methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
					let formatted_enum = format!("{:?}", this.0);

					formatted_enum.to_lua(ctx)
				});
			}
		}
	};

	gen.into()
}

#[proc_macro_derive(TBBitflag)]
pub fn tb_bitflag(tokens: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse(tokens.clone()).unwrap();

	let name = ast.ident;

	let data_struct: DataStruct = match ast.data {
		syn::Data::Struct(data) => data,
		_ => panic!("Failed to parse struct for TBBitflag Macro"),
	};

	let mut tuple_fields: Vec<&Ident> = Vec::new();

	for field in data_struct.fields.iter() {
		let ident: syn::Result<&Ident> = match &field.ty {
			syn::Type::Path(path) => {
				let ident = path.path.get_ident().unwrap();

				Ok(ident)
			}
			_ => panic!("Failed to parse path and ident for struct wrapper."),
		};

		let ident_uw = ident.unwrap();

		tuple_fields.push(ident_uw);
	}

	// TODO: Add other methods and struct consts of a bitflags object such as https://docs.rs/serenity/latest/serenity/model/permissions/struct.Permissions.html

	quote! {
		use rlua::{UserData, Value, MetaMethod, ToLua};

		impl UserData for #name {
			fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
				methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
					this.0.bits().to_string().to_lua(ctx)
				})
			}
		}
	}
	.into()
}

#[proc_macro_attribute]
pub fn tb_names(args: TokenStream, tokens: TokenStream) -> TokenStream {
	let args: syn::AttributeArgs = parse_macro_input!(args as syn::AttributeArgs);
	let ast: syn::DeriveInput = syn::parse(tokens.clone()).unwrap();

	let arg = args.get(0).unwrap();

	let fn_name: String = match arg {
		syn::NestedMeta::Lit(lit) => match lit {
			syn::Lit::Str(litstr) => litstr.token().to_string(),
			_ => panic!("Literal string not found"),
		},
		_ => panic!("String not found"),
	};

	println!("fn_name {:#?}", fn_name);

	let name = ast.ident;

	quote! {
		trait GetNames {
			fn get_names(&self) -> Vec::<String>;
		}

		impl GetNames for #name {
			fn get_names(&self) -> Vec::<String> {
				self.0. #fn_name ()
			}
		}
	}
	.into()
}
