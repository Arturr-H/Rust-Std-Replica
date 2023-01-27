/* Imports */
mod hashmap;

fn main() {
    let mut hm = hashmap::HashMap::new();
    hm.insert("tom", 15);
    hm.insert("max", 12);
    hm.insert("linda", 172);
    hm.insert("bob", 111);
    hm.insert("janice", 222);
    hm.insert("haha", 333);
    hm.insert("popo", 444);
    hm.insert("baba", 555);
    println!("{hm:?}");
    println!("{:?}", hm.get("janice"));
}
