// This is stolen from serenity, lol

use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse::{ParseStream, Parse, Result as SynResult, Error}, punctuated::Punctuated, token::Comma, parenthesized};

#[derive(Debug)]
pub struct Parenthesised<T>(pub Punctuated<T, Comma>);

impl<T: Parse> Parse for Parenthesised<T> {
    fn parse(input: ParseStream<'_>) -> SynResult<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Parenthesised(content.parse_terminated(T::parse)?))
    }
}

#[inline]
pub fn into_stream(e: &Error) -> TokenStream {
    e.to_compile_error().into()
}


macro_rules! propagate_err {
    ($res:expr) => {{
        match $res {
            Ok(v) => v,
            Err(e) => return $crate::util::into_stream(&e),
        }
    }};
}

pub trait IdentExt2: Sized {
    fn to_string_non_raw(&self) -> String;
    fn to_uppercase(&self) -> Self;
    fn with_suffix(&self, suf: &str) -> Ident;
}

impl IdentExt2 for Ident {
    #[inline]
    fn to_string_non_raw(&self) -> String {
        let ident_string = self.to_string();
        ident_string.trim_start_matches("r#").into()
    }

    #[inline]
    fn to_uppercase(&self) -> Self {
        // This should be valid because keywords are lowercase.
        format_ident!("{}", self.to_string_non_raw().to_uppercase())
    }

    #[inline]
    fn with_suffix(&self, suffix: &str) -> Ident {
        format_ident!("{}_{}", self.to_uppercase(), suffix)
    }
}
