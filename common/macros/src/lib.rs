use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, bracketed, parenthesized, parse_macro_input, Expr, Ident, LitInt, LitStr, Result,
    Signature, Token, Type,
};

// will print the AST every time a game class is instantiated with the set prefix.
// mostly useful for debugging what the produced AST is.
const DEBUG_PRINT_CLASS_WITH_PREFIX: &str = "";

mod kw {
    syn::custom_keyword!(size);
    syn::custom_keyword!(location);
    syn::custom_keyword!(fields);
    syn::custom_keyword!(attributes);
    syn::custom_keyword!(functions);
    syn::custom_keyword!(signature);
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

enum LocationType {
    Signature(String),
}

struct Function {
    location: LocationType,
    signature: Signature,
}
impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        bracketed!(content in input);

        content.parse::<kw::signature>()?;

        let signature_contents;
        parenthesized!(signature_contents in content);
        let code_signature = signature_contents.parse::<LitStr>()?.value();
        let signature = input.parse::<Signature>()?;

        Ok(Function {
            location: LocationType::Signature(code_signature),
            signature,
        })
    }
}

enum Property {
    Size(LitInt),
    Location(Expr),
    Fields(Punctuated<Field, Token![,]>),
    Attributes(Vec<syn::Attribute>),
    Functions(Punctuated<Function, Token![;]>),
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
        } else if lookahead.peek(kw::attributes) {
            input.parse::<kw::attributes>()?;
            input.parse::<Token![:]>()?;

            let attributes = input.call(syn::Attribute::parse_outer)?;
            Ok(Property::Attributes(attributes))
        } else if lookahead.peek(kw::functions) {
            input.parse::<kw::functions>()?;
            input.parse::<Token![:]>()?;

            let content;
            braced!(content in input);

            Ok(Property::Functions(
                content.parse_terminated(Function::parse)?,
            ))
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
    attributes: Vec<syn::Attribute>,
    functions: Vec<Function>,
}

fn retrieve_properties(
    properties: Punctuated<Property, Token![,]>,
) -> syn::Result<RetrievedProperties> {
    let mut size: Option<u32> = None;
    let mut location: Option<Expr> = None;
    let mut fields: Vec<(u32, Ident, Type)> = vec![];
    let mut attributes: Vec<syn::Attribute> = vec![];
    let mut functions: Vec<Function> = vec![];

    for property in properties {
        match property {
            Property::Size(v) => {
                size = Some(v.base10_parse()?);
            }
            Property::Location(e) => {
                location = Some(e);
            }
            Property::Fields(fs) => {
                fields = fs
                    .into_iter()
                    .map(|f| (f.offset.base10_parse().unwrap(), f.name, f.type_expr))
                    .collect();
            }
            Property::Attributes(attrs) => {
                attributes = attrs;
            }
            Property::Functions(fs) => {
                functions = fs.into_iter().collect();
            }
        }
    }

    Ok(RetrievedProperties {
        size,
        location,
        fields,
        attributes,
        functions,
    })
}

fn generate_fields(
    size: Option<u32>,
    _fields: &[(u32, Ident, Type)],
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    // We used to generate fields here, but we've disabled that after the Rust compiler
    // deprecated taking unaligned references of fields. They're not much use if you
    // can't use them directly.
    let mut fields_calculated: Vec<proc_macro2::TokenStream> = vec![];

    if let Some(size) = size.map(|x| x as usize) {
        fields_calculated.push(quote! { __pad: [u8; #size] });
    }

    Ok(fields_calculated)
}

fn generate_get(name: &proc_macro2::Ident, location: Option<Expr>) -> proc_macro2::TokenStream {
    match location {
        Some(e) => quote! {
            pub unsafe fn get() -> &'static mut #name {
                let m = crate::util::game_module().unwrap();
                let p: *mut #name = *(m.rel_to_abs_addr(#e as isize) as *const *mut #name);
                &mut *p
            }
        },
        None => quote! {},
    }
}

fn generate_getters(fields: &[(u32, Ident, Type)]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|(offset, name, field_type)| {
            let offset = *offset as usize;

            let name_ptr = format_ident!("{}_ptr", name);
            let name_ptr_mut = format_ident!("{}_ptr_mut", name);

            let ref_methods = if offset % 4 == 0 {
                let name = format_ident!("{}", name);
                let name_mut = format_ident!("{}_mut", name);

                quote! {
                    pub unsafe fn #name(&self) -> &'static #field_type {
                        &*self.#name_ptr()
                    }
                    pub unsafe fn #name_mut(&mut self) -> &'static mut #field_type {
                        &mut *self.#name_ptr_mut()
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                pub unsafe fn #name_ptr(&self) -> *const #field_type {
                    let u8_self = self as *const _ as *const u8;
                    (self as *const _ as *const u8).add(#offset) as *const #field_type
                }
                pub unsafe fn #name_ptr_mut(&mut self) -> *mut #field_type {
                    let u8_self = self as *mut _ as *mut u8;
                    (self as *mut _ as *mut u8).add(#offset) as *mut #field_type
                }

                #ref_methods
            }
        })
        .collect()
}

fn generate_functions(
    type_name: &proc_macro2::Ident,
    functions: &[Function],
) -> Vec<proc_macro2::TokenStream> {
    functions
        .iter()
        .map(|f| {
            let name = &f.signature.ident;
            let args = f.signature.inputs.iter().collect::<Vec<_>>();
            let args_type = f
                .signature
                .inputs
                .iter()
                .map(|a| match a {
                    syn::FnArg::Receiver(r) => {
                        let mutability = r.mutability;
                        quote! { &#mutability #type_name }
                    }
                    syn::FnArg::Typed(p) => p.ty.to_token_stream(),
                })
                .collect::<Vec<_>>();
            let args_call = f
                .signature
                .inputs
                .iter()
                .map(|a| match a {
                    syn::FnArg::Receiver(r) => r.self_token.to_token_stream(),
                    syn::FnArg::Typed(p) => p.pat.to_token_stream(),
                })
                .collect::<Vec<_>>();

            let output = &f.signature.output;
            let LocationType::Signature(code_signature) = &f.location;

            quote! {
                pub unsafe fn #name(#(#args, )*) #output {
                    static mut ADDRESS: *mut u8 = ::std::ptr::null_mut();
                    if ADDRESS.is_null() {
                        let module = crate::util::game_module().unwrap();
                        ADDRESS = module.scan(#code_signature).unwrap();
                    }

                    type FunctionType = extern "system" fn(#(#args_type), *) #output;
                    let f: FunctionType = ::std::mem::transmute(ADDRESS);
                    f(#(#args_call, )*)
                }
            }
        })
        .collect()
}

#[proc_macro]
pub fn game_class(item: TokenStream) -> TokenStream {
    let GameClass { name, properties } = parse_macro_input!(item as GameClass);
    let RetrievedProperties {
        size,
        location,
        fields,
        attributes,
        functions,
    } = retrieve_properties(properties).unwrap();

    let fields_impl = generate_fields(size, &fields).unwrap();
    let get_impl = generate_get(&name, location);
    let getters_impl: Vec<_> = generate_getters(&fields);
    let functions_impl: Vec<_> = generate_functions(&name, &functions);

    let expanded = quote! {
        #[repr(C, packed(1))]
        #[allow(dead_code)]
        #(#attributes )*
        pub struct #name {
            #(#fields_impl,)*
        }

        impl #name {
            #get_impl
            #(#getters_impl)*
            #(#functions_impl)*
        }
    };

    // will print the AST every time a game class is instantiated with the set prefix.
    // mostly useful for debugging what the produced AST is.
    if !DEBUG_PRINT_CLASS_WITH_PREFIX.is_empty()
        && name.to_string().starts_with(DEBUG_PRINT_CLASS_WITH_PREFIX)
    {
        println!("{}", expanded);
    }

    TokenStream::from(expanded)
}
