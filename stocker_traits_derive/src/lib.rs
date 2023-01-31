use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Stock)]
pub fn stock_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_stock_macro(&ast)
}

fn impl_stock_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Stock for #name {}
    };
    gen.into()
}
