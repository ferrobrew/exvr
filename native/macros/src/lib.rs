use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, bracketed, parse_macro_input, parse_quote, Expr, Ident, LitInt, Result, Token, Type,
};

mod kw {
    syn::custom_keyword!(size);
    syn::custom_keyword!(location);
    syn::custom_keyword!(fields);
}

struct Field {
    offset: LitInt,
    name: Ident,
    type_expr: Type,
}
impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        bracketed!(content in input);
        let offset: LitInt = content.parse()?;

        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_expr: Type = input.parse()?;

        Ok(Field {
            offset,
            name,
            type_expr,
        })
    }
}

enum Property {
    Size(LitInt),
    Location(Expr),
    Fields(Punctuated<Field, Token![,]>),
}
impl Parse for Property {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::size) {
            input.parse::<kw::size>()?;
            input.parse::<Token![:]>()?;
            Ok(Property::Size(input.parse()?))
        } else if lookahead.peek(kw::location) {
            input.parse::<kw::location>()?;
            input.parse::<Token![:]>()?;
            Ok(Property::Location(input.parse()?))
        } else if lookahead.peek(kw::fields) {
            input.parse::<kw::fields>()?;
            input.parse::<Token![:]>()?;

            let content;
            braced!(content in input);

            Ok(Property::Fields(content.parse_terminated(Field::parse)?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct GameClass {
    name: Ident,
    properties: Punctuated<Property, Token![,]>,
}

impl Parse for GameClass {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let content;
        braced!(content in input);

        let properties: Punctuated<Property, Token![,]> =
            content.parse_terminated(Property::parse)?;

        Ok(GameClass { name, properties })
    }
}

struct RetrievedProperties {
    size: Option<u32>,
    location: Option<Expr>,
    fields: Vec<(u32, Ident, Type)>,
}

fn retrieve_properties(
    properties: Punctuated<Property, Token![,]>,
) -> syn::Result<RetrievedProperties> {
    let mut size: Option<u32> = None;
    let mut location: Option<Expr> = None;
    let mut fields: Vec<(u32, Ident, Type)> = vec![];

    for property in properties {
        match property {
            Property::Size(v) => {
                size = Some(v.base10_parse()?);
            }
            Property::Location(e) => {
                location = Some(e);
            }
            Property::Fields(fs) => {
                for f in fs {
                    fields.push((f.offset.base10_parse()?, f.name, f.type_expr))
                }
            }
        }
    }

    Ok(RetrievedProperties {
        size,
        location,
        fields,
    })
}

fn retrieve_fields(
    size: Option<u32>,
    fields: Vec<(u32, Ident, Type)>,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields_calculated: Vec<proc_macro2::TokenStream> = vec![];

    let mut index = 0u32;
    let mut last_offset = 0u32;
    let mut last_type: Type = parse_quote! { () };
    for (offset, name, type_expr) in fields {
        let delta = offset - last_offset;
        let pad_ident = format_ident!("__pad{}", index);
        fields_calculated
            .push(quote! { #pad_ident: [u8; #delta as usize - std::mem::size_of::<#last_type>()] });

        last_type = type_expr.clone();
        last_offset = offset;

        fields_calculated.push(quote! { pub #name: #type_expr });
        index += 1;
    }

    if let Some(size) = size {
        let delta = size - last_offset;
        let pad_ident = format_ident!("__pad{}", index);
        fields_calculated
            .push(quote! { #pad_ident: [u8; #delta as usize - std::mem::size_of::<#last_type>()] });
    }

    Ok(fields_calculated)
}

#[proc_macro]
pub fn game_class(item: TokenStream) -> TokenStream {
    let GameClass { name, properties } = parse_macro_input!(item as GameClass);
    let RetrievedProperties {
        size,
        location,
        fields,
    } = retrieve_properties(properties).unwrap();

    let fields_impl = retrieve_fields(size, fields).unwrap();

    let get_impl = if let Some(e) = location {
        quote! {
            impl #name {
                pub fn get() -> &'static mut #name {
                    unsafe { 
                        let m = crate::module::GAME_MODULE.get().unwrap();
                        let p: *mut #name = *(m.rel_to_abs_addr(#e as isize) as *const *mut #name);
                        &mut *p
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #[repr(C)]
        pub struct #name {
            #(#fields_impl,)*
        }

        #get_impl
    };

    TokenStream::from(expanded)
}
