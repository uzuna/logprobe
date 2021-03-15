extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 操作可能な抽象構文木としてのRustコードを構築
    let ast = syn::parse(input).unwrap();

    // 構文木を操作してimplを実装する
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
            fn hello_method(&self) -> String{
                format!("Hello, Macro! My name is {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}
