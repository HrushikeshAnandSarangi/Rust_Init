#![warn(missing_debug_implementations,rust_2018_idioms,missing_docs)]

use std::vec;


pub struct StrSplit{
    remainder:&str,
    delimiter:&str,
}

impl StrSplit {
    pub fn new(haystack:&str,delimiter:&str)->Self{
    Self { remainder: haystack, delimiter, }
    }
    // add code here
}
//x:StrSplit;
//for part in x{
//
//}
impl Iterator for StrSplit{
    type  Item=&str ;
    fn next(&mut self)->Option<self::Item>{
        if let some(next_delim)=self.remainder.find(self.delimiter) {
            let until_delimiter=&self.remainder[..next_delim];
            self.remainder=&self.remainder[(next_delim+self.delimiter.len())..];
            Some(until_delimiter)
            
        }self.remainder.is_empty() {
            None
        }else{
            let rest=self.remainder;
            self.remainder=&[];
            Some(rest)
        }
    }
}

#[test]
fn it_works(){
    let haystack="a b c d e";
    for letter in StrSplit::new(haystack, " ");
    assert_eq!(letter,vec!["a","b","c","d","e"].into_iter());
}
