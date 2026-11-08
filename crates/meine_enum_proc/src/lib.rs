mod flag;
use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{LitInt, Ident, Token, parse::{Parse, ParseStream}, parse_macro_input};


use flag::flag_impl;


#[cfg(feature="standard")]
#[proc_macro_attribute]
pub fn flag(attr: TokenStream, item: TokenStream) -> TokenStream {flag_impl("flag", attr, item)}

#[cfg(not(feature="standard"))]
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Flag(attr: TokenStream, item: TokenStream) -> TokenStream {flag_impl("Flag", attr, item)}

struct Input {
    name: Ident,
    number: LitInt,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;
        let number = input.parse()?;

        Ok(Self { name, number })
    }
}

#[cfg(feature="test")]
#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream {
    let Input { name, number } = parse_macro_input!(input as Input);

    let value: u128 = number.base10_parse().unwrap();
    
    let mut constants = vec![];
    for i in 0..value {
        let ident = format_ident!("A{i}");
        constants.push(quote! {
            #ident
        })
    }

    quote! {
        #[Flag]
        #[derive(Debug)]
        enum #name {
            #(#constants),*
        }
    }.into()
}
