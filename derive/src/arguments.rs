use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub struct MetaAttribute {
    pub name: syn::Ident,
    pub assign: Token![=],
    pub args: MetaArgs,
}

pub struct MetaArgs {
    pub paren: syn::token::Paren,
    pub args: Punctuated<MetaArg, Token![,]>,
}

pub struct MetaArg {
    pub name: syn::Ident,
    pub value: Option<(Token![=], syn::Lit)>,
}

impl Parse for MetaAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MetaAttribute {
            name: input.parse()?,
            assign: input.parse()?,
            args: input.parse()?,
        })
    }
}

impl Parse for MetaArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        let paren = parenthesized!(content in input);
        let args = content.parse_terminated(MetaArg::parse)?;
        Ok(MetaArgs { paren, args })
    }
}

impl Parse for MetaArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MetaArg {
            name: input.parse()?,
            value: {
                if input.peek(Token![=]) {
                    Some((input.parse()?, input.parse()?))
                } else {
                    None
                }
            },
        })
    }
}
