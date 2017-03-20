extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::collections::HashSet;

use proc_macro::TokenStream;

#[proc_macro_derive(Packed)]
pub fn derive_parsed(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    match impl_parsed(&ast) {
    	Ok(gen) => gen.parse().unwrap(),
		Err(msg) => panic!(msg)
	}
}

fn impl_parsed(ast: &syn::DeriveInput) -> Result<quote::Tokens, String> {
    let name = &ast.ident;
    let mut type_set = HashSet::<syn::Ty>::new();
    let mut members = Vec::<syn::Ident>::new();
    if let syn::Body::Enum(_) = ast.body {
        return Err("Cannot implement Parsed for an enum".to_string());
    }
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        for ref field in fields {
            type_set.insert(field.ty.clone());
            members.push(field.ident.clone().unwrap());
        }
    }
    Ok(quote! {
        impl Packed for #name where #( #type_set : Packed ),* {
            fn switch_endianness(&mut self) {
                #( self.#members.switch_endianness(); )*
            }
        }
    })
}
