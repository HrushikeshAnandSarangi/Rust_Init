use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn show_streams(attr:TokenStream,item:TokenStream)->TokenStream{
    println!("--Attribute Tokens --\n{}\n",attr.to_string());
    println!("--Item Tokens --\n{}\n",item.to_string());
    item
}
