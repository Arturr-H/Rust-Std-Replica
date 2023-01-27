/* Global allowings */
#![allow(dead_code)]

/* Imports */
mod hashmap;
mod string;

fn main() {
    let mut hm = hashmap::HashMap::new();
    hm.insert("tom", 15);
    println!("{:?}", hm.get("tom"));
    let string = string::StdString::from("hello!");
    println!("{}", &string[0..2]);
}
