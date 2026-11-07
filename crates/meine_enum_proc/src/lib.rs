mod flag;
use proc_macro::TokenStream;

use flag::flag_impl;


#[cfg(feature="standard")]
#[proc_macro_attribute]
pub fn flag(attr: TokenStream, item: TokenStream) -> TokenStream {flag_impl("flag", attr, item)}

#[cfg(not(feature="standard"))]
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Flag(attr: TokenStream, item: TokenStream) -> TokenStream {flag_impl("Flag", attr, item)}
