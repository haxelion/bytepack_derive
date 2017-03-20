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
    	Ok(gen) => {
            gen.parse().unwrap()
        }
		Err(msg) => panic!(msg)
	}
}

fn impl_parsed(ast: &syn::DeriveInput) -> Result<quote::Tokens, String> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let mut where_clause = where_clause.clone();
    let mut type_set = HashSet::<syn::Ty>::new();
    let mut members = Vec::<syn::Ident>::new();

    match ast.body {
        syn::Body::Enum(_) =>  {
            return Err("Cannot implement Parsed for an enum".to_string());
        },
        syn::Body::Struct(syn::VariantData::Unit) => {
            return Err("Cannot implement Parsed for a unit struct".to_string());
        }
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            for ref field in fields {
                type_set.insert(field.ty.clone());
                members.push(field.ident.clone().unwrap());
            }
        },
        syn::Body::Struct(syn::VariantData::Tuple(ref fields)) => {
            for i in 0..fields.len() {
                type_set.insert(fields[i].ty.clone());
                members.push(syn::Ident::new(i));
            }
        }
    }
    if members.len() == 0 {
            return Err("Cannot implement Parsed for an empty struct".to_string());
    }

    let packed_bound = syn::parse_ty_param_bound("bytepack::Packed")?;

    for ty in &type_set {
        where_clause.predicates.push(syn::WherePredicate::BoundPredicate(syn::WhereBoundPredicate {
            bound_lifetimes: Vec::new(),
            bounded_ty: ty.clone(),
            bounds: vec![packed_bound.clone()]
        }));
    }

    Ok(quote! {
        impl #impl_generics Packed for #name #ty_generics #where_clause {
            fn switch_endianness(&mut self) {
                #( self.#members.switch_endianness(); )*
            }
        }
    })
}
