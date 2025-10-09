use proc_macro::TokenStream;
use syn::{parse_macro_input,LitStr};
use quote::quote;

#[proc_macro]
pub fn sql(input:TokenStream)->TokenStream{
    let sql_string=parse_macro_input!(input as LitStr);
    println!("-> Compiling SQL:'{}'",sql_string.value());
    let expanded=quote!{
        #sql_string
    };
    expanded.into()
}
