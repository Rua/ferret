extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Asset, attributes(storage))]
pub fn asset(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	let gen = impl_asset(&ast);
	gen.into()
}

fn impl_asset(ast: &DeriveInput) -> proc_macro2::TokenStream {
	let name = &ast.ident;
	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

	quote! {
		impl #impl_generics Asset for #name #ty_generics #where_clause {
			type Data = Self;
		}
	}
}
