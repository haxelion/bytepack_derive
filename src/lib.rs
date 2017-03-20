extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Parsed)]
pub fn derive_parsed(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    match impl_parsed(&ast) {
    	Ok(gen) => gen.parse().unwrap(),
		Err(msg) => panic!(msg)
	}
}

fn impl_parsed(ast: &syn::MacroInput) -> Result<quote::Tokens, String> {
    let name = &ast.ident;
    Ok(quote! {
        impl Parsed for #name {
            fn endianness_switch(&mut self) {
            }
        }
    })
}
