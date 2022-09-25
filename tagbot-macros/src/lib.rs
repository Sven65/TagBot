use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{self, parse::{ParseStream, Parse}, Token, ItemEnum, token::{self, Token, Struct, Enum}, punctuated::Punctuated, Field, Result, braced, parse_macro_input, DataStruct};
use quote::quote;


#[proc_macro_derive(LuaEnum)]
pub fn lua_enum(tokens: TokenStream) -> TokenStream {
    println!("Derive LuaEnum");
    
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    println!("Ast is {:#?}", ast.data);

    let name = ast.ident;

    let data_struct: DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _=> panic!("Failed to parse struct for LuaEnum Macro"),
    };

    println!("data_struct {:#?}", data_struct);

    for field in data_struct.fields.iter() {
        let ty = &field.ty;
        println!("Field type is {:#?}", ty);

        let path: syn::Result<String> = match &field.ty {
            syn::Type::Path(path) => {
                let ident = path.path.get_ident();
                println!("Type thing, {:#?}", ident.unwrap());
                println!("type thing 2 {:#?}", ident.unwrap().)
                println!("type path {:#?}", path);
                let path_strings: Vec<String> = (&path.path.segments).into_iter().map(|segment| {
                    segment.ident.to_string()
                }).collect();
    
                let final_string = path_strings.join("::");
    
                Ok(final_string)
            },
            _ => panic!("Failed to parse path for struct member."),
        };

        let parsed = syn::parse_str::<Enum>(path.unwrap().as_str());

        println!("Parsed into {:#?}", parsed);
    }

    let gen = quote! {
        impl UserData for #name {
            fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
                methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
                    let level = match this.0 {
                        MfaLevel::None => "None",
                        MfaLevel::Elevated => "Elevated",
                        MfaLevel::Unknown => "Unknown",
                        _ => "Unknown",
                    };
        
                    Ok(level.to_lua(ctx)?)
                });
            }
        }
    };

    gen.into()

}