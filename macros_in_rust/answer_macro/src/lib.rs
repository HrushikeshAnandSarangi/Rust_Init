use proc_macro::TokenStream;
use quote::quote;
use syn::{self,DeriveInput,parse_macro_input};

#[proc_macro_derive(Answer)]
pub fn answer_macro_derive(input:TokenStream)->TokenStream{
    let ast = parse_macro_input!(input as DeriveInput);//ast->Abstract Syntax Tree
    impl_answer_macro(&ast)
}

fn impl_answer_macro(ast:&syn::DeriveInput)->TokenStream {
    let name=&ast.ident;
    let generated=quote!{
        impl Answer for #name {
            fn answer()->u32{
                42
            }
            // add code here
        }
    };
    generated.into()
    
}
