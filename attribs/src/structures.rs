use syn::{
    braced,
    Attribute,
    Block,
    FnArg,
    Ident,
    Stmt,
    Token,
    parse::Parse,
	Lit,
};

use crate::util::Parenthesised;

pub trait LitExt {
    fn to_str(&self) -> String;
    fn to_bool(&self) -> bool;
    fn to_ident(&self) -> Ident;
}

impl LitExt for Lit {
    fn to_str(&self) -> String {
        match self {
            Self::Str(s) => s.value(),
            Self::ByteStr(s) => unsafe { String::from_utf8_unchecked(s.value()) },
            Self::Char(c) => c.value().to_string(),
            Self::Byte(b) => (b.value() as char).to_string(),
            _ => panic!("values must be a (byte)string or a char"),
        }
    }

    fn to_bool(&self) -> bool {
        if let Lit::Bool(b) = self {
            b.value
        } else {
            self.to_str().parse().unwrap_or_else(|_| panic!("expected bool from {:?}", self))
        }
    }

    #[inline]
    fn to_ident(&self) -> Ident {
        Ident::new(&self.to_str(), self.span())
    }
}



#[derive(Debug)]
pub struct CommandFun {
	/// `#[...]`-style attribs
	pub attributes: Vec<Attribute>,

	pub name: Ident,
	pub body: Vec<Stmt>,
}

impl Parse for CommandFun {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let attributes = input.call(Attribute::parse_outer)?;

		input.parse::<Token![async]>()?;
		input.parse::<Token![fn]>()?;
        let name = input.parse()?;

		// (...)
		let Parenthesised(_args) = input.parse::<Parenthesised<FnArg>>()?;

		// let ret = match input.parse::<ReturnType>()? {
        //     ReturnType::Type(_, t) => (*t).clone(),
        //     ReturnType::Default => {
        //         return Err(input
        //             .error("expected a result type of either `CommandResult` or `CheckResult`"))
        //     },
        // };

		let bcont;
        braced!(bcont in input);
        let body = bcont.call(Block::parse_within)?;

		Ok(Self {
			attributes,
			name,
			body,
		})
	}
}

#[derive(Debug, Default)]
pub struct Options {
	pub description: String,
}

impl Options {
    #[inline]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}