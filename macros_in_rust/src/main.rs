use answer_macro::Answer;
use sql_macro::sql;
use attribute_macro::show_streams;
macro_rules! create_vec {
    ($($item:expr),*) => {
        {
            let mut temp_vec=Vec::new();
            $(
                temp_vec.push($item);
            )*
                temp_vec
        }
    };
}
macro_rules! create_struct_with_field {
    ($struct_name:ident,$field_name:ident,$field_type:ty) => {
        struct $struct_name{
            $field_name:$field_type,
        }
        
        
    };
}
trait Answer {
    fn answer()->u32;
    
}
#[derive(Answer)]
struct MyStruct;

#[show_streams(some,arguments)]
fn attribute_function() {

    
}
fn main() {
    let  my_vec = create_vec![1,2,3] ;
    println!("{:?}",my_vec);
    let string_vec = create_vec!["Hello","World"] ;
    println!("{:?}",string_vec);

    create_struct_with_field!(Product,price,f64);
    let laptop=Product{price:12000.34};
    println!("The product's price is:{}",laptop.price);
    println!("The answer is {}",MyStruct::answer());
    let my_query=sql!("SELECT * FROM users;");
    println!("->Executing query: {}",my_query);
}

