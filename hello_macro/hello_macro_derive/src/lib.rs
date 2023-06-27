use proc_macro::TokenStream;
use quote::quote;
use syn;

// with this annotation, the hello_macro_derive func can be called whenever a user specifies #[derive(HelloMacro)] on a type
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // the above parse method returns a unit struct like this: (ident is short for identifier)
    //     DeriveInput {
    //     // --snip--

    //     ident: Ident {
    //         ident: "Pancakes",
    //         span: #0 bytes(95..103)
    //     },
    //     data: Struct(
    //         DataStruct {
    //             struct_token: Struct,
    //             fields: Unit,
    //             semi_token: Some(
    //                 Semi
    //             )
    //         }
    //     )
    // }

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
