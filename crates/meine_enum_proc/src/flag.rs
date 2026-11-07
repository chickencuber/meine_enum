use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use std::fmt::Display;

pub fn flag_impl<T: Display>(name: T, attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            format_args!("{name} does not accept arguments"),
        )
            .to_compile_error()
            .into();
    }
    let input = syn::parse_macro_input!(item as syn::Item);

    let item_enum = match input {
        syn::Item::Enum(item_enum) => item_enum,
        _ => {
            return syn::Error::new_spanned(input, format_args!("{name} Expected an enum"))
                .to_compile_error()
                .into();
            }
    };
    if item_enum.variants.is_empty() {
        return syn::Error::new_spanned(
            &item_enum,
            format_args!("{name} enums must have at least one variant"),
        )
            .to_compile_error()
            .into();
    }
    let mut has_debug = false;
    let mut attrs = item_enum.attrs;

    attrs.retain_mut(|attr| {
        if attr.path().is_ident("derive") {
            if let Ok(derives) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) {
                let derives: syn::punctuated::Punctuated<_, syn::Token![,]> = derives
                    .into_iter()
                    .filter(|path| {
                        if path.is_ident("Debug") {
                            has_debug = true;
                            false
                        } else if path.is_ident("Clone")
                            || path.is_ident("Copy")
                                || path.is_ident("PartialEq")
                                || path.is_ident("Eq")
                        {
                            false
                        } else {
                            true
                        }
                    })
                .collect();
                // Keep the attribute only if derives remain.
                if derives.is_empty() {
                    return false;
                }
                attr.meta = syn::parse_quote!(derive(#derives));
            }
        }
        true
    });
    for variant in &item_enum.variants {
        if !matches!(variant.fields, syn::Fields::Unit) {
            return syn::Error::new_spanned(
                variant,
                format_args!("{name} enums cannot have fields"),
            )
                .to_compile_error()
                .into();
        }
    }
    if !item_enum.generics.params.is_empty() {
        return syn::Error::new_spanned(
            &item_enum.generics,
            format_args!("{name} enums cannot have generics"),
        )
            .to_compile_error()
            .into();
    }

    let vis = item_enum.vis.clone();
    let name = format!(
        "__tags__{}__mod__",
        item_enum.ident.to_string().to_snake_case()
    );
    let ident = item_enum.ident;
    let mod_ident = syn::Ident::new(name.as_str(), proc_macro2::Span::call_site());

    let len = item_enum.variants.len();
    let ty = match len {
        0..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        65..=128 => quote!(u128),
        _ => todo!("support more than 128 flags"), //TASK:(20261106-205910-520-n6-360): make it work with more than 128 variants
    };

    let mut types = vec![];

    let constants: Vec<_> = item_enum
        .variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let val: u128 = 1 << i;
            let value = syn::LitInt::new(&val.to_string(), proc_macro2::Span::call_site());
            let v_ident = &variant.ident;
            types.push(quote!{
                #v_ident
            });
            quote! {
                pub const #v_ident: Self = Self(#value);
            }
        })
    .collect();

    let debug = if !has_debug {quote!{}} else {quote!{
        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}(", stringify!(#ident))?;
                let mut first = true;
                for i in *self {
                    if !first {
                        write!(f, " | ")?;
                    }
                    #(
                    if i == Self::#types {
                        write!(f, stringify!(#types))?;
                        first = false;
                        continue;
                    }
                    )*
                }
                write!(f, ")")
            }
        }
    }};

    quote! {
        mod #mod_ident {
            #(#attrs)*
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct #ident(#ty);
            impl #ident {
                #(#constants)*

                /// Returns a flag set with no flags enabled.
                pub fn empty() -> Self {
                    Self(0)
                }

                /// Returns a flag set with all defined flags enabled.
                pub fn all() -> Self {
                    #(Self::#types)|*
                }

                /// Returns whether all bits in `other` are present.
                pub fn contains(&self, other: Self) -> bool {
                    (other - *self).0 == 0
                }

                /// Returns whether at least one bit overlaps with `other`.
                pub fn intersects(&self, other: Self) -> bool {
                    (*self & other).0 != 0
                }
                /// Returns whether no flags are enabled.
                pub fn is_empty(&self) -> bool {
                    self.0 == 0
                }
                /// Returns whether all defined flags are enabled.
                pub fn is_all(&self) -> bool {
                    *self == Self::all()
                }
                /// Sets or clears the flags in `other` depending on `value`.
                pub fn set(&mut self, other: Self, value: bool) {
                    if value {
                        (*self) |= other
                    } else {
                        (*self) -= other
                    }
                }
                /// Enables all defined flags.
                pub fn set_all(&mut self) {
                    self.0 = Self::all().0;
                }
                /// Disables all flags.
                pub fn clear_all(&mut self) {
                    self.0 = 0;
                }
                /// Inverts all defined flags in place.
                pub fn invert(&mut self) {
                    self.0 = !self.0;
                    *self &= Self::all();
                }
            }


            impl std::ops::BitAnd<#ident> for #ident {
                type Output = #ident;
                fn bitand(self, rhs: #ident) -> Self::Output {
                    Self(self.0 & rhs.0)
                }
            }
            impl std::ops::BitOr<#ident> for #ident {
                type Output = #ident;
                fn bitor(self, rhs: #ident) -> Self::Output {
                    Self(self.0 | rhs.0)
                }
            }
            impl std::ops::BitXor<#ident> for #ident {
                type Output = #ident;
                fn bitxor(self, rhs: #ident) -> Self::Output {
                    Self(self.0 ^ rhs.0)
                }
            }
            impl std::ops::Sub<#ident> for #ident {
                type Output = #ident;
                fn sub(self, rhs: #ident) -> Self::Output {
                    Self(self.0 & !rhs.0)
                }
            }
            impl std::ops::Not for #ident {
                type Output = #ident;
                fn not(self) -> Self::Output {
                    Self(!self.0)&Self::all()
                }
            }
            impl std::ops::BitAndAssign<#ident> for #ident {
                fn bitand_assign(&mut self, rhs: #ident) {
                    self.0 = self.0 & rhs.0;
                }
            }
            impl std::ops::BitOrAssign<#ident> for #ident {
                fn bitor_assign(&mut self, rhs: #ident) {
                    self.0 = self.0 | rhs.0;
                }
            }
            impl std::ops::BitXorAssign<#ident> for #ident {
                fn bitxor_assign(&mut self, rhs: #ident) {
                    self.0 = self.0 ^ rhs.0;
                }
            }
            impl std::ops::SubAssign<#ident> for #ident {
                fn sub_assign(&mut self, rhs: #ident) {
                    self.0 = self.0 & !rhs.0;
                }
            }

            impl meine_enum::ToFlagIter for #ident {
                fn next(&mut self) -> Option<Self> {
                    #(
                        if self.intersects(Self::#types) {
                            *self -= Self::#types;
                            return Some(Self::#types);
                        }
                    )*
                        return None
                }
            }
            
            impl IntoIterator for #ident {
                type Item = #ident;
                type IntoIter = meine_enum::FlagIter<#ident>;
                fn into_iter(self) -> Self::IntoIter {
                    meine_enum::FlagIter::new(self)
                }
            }
            #debug
        }
        #vis use #mod_ident::#ident;
    }
    .into()
}
