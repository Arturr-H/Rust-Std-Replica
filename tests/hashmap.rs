#[cfg(test)]
mod tests {
    /* Imports */
    use practicing_rust::hashmap::HashMap;

    /* Test functions */
    #[test]
    fn construct_hashmap() -> () {
        let hm:HashMap<(), ()> = HashMap::new();
        dbg!(hm.len());
        assert_eq!(hm.len() == 10, true);
        
        let hm_custom:HashMap<(), ()> = HashMap::with_capacity(15);
        assert_eq!(hm_custom.len() == 15, true);
    }

    #[test]
    fn insert_hashmap() -> () {
        let mut hm = HashMap::new();
        hm.insert("key", 921578128);
        hm.insert("second", 51212);

        assert_eq!(hm.get("key") == Some(&921578128), true);
        assert_eq!(hm.get("second") == Some(&51212), true);
    }
}