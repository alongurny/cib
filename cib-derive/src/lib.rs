use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Derive macro for `Cib` trait. Generates a trivial impl that treats the
/// owned type as itself (moves when owned). For a type `T`, this expands to:
#[proc_macro_derive(Cib)]
pub fn derive_cib(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::cib::Cib<Self> for #name #ty_generics #where_clause {
            fn cib(self) -> Self { self }
        }
    };

    expanded.into()
}
