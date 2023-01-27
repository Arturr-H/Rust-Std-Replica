#[cfg(test)]
mod tests {
    /* Imports */
    use practicing_rust::string::StdString;

    /* Test functions */
    #[test]
    fn construct_string() -> () {
        let string:StdString = StdString::new();
        assert_eq!(string.as_bytes().len() == 0, true);
        assert_eq!(string.is_empty(), true);

        let string2:StdString = StdString::from("value");
        assert_eq!(string2.as_bytes() == "value".as_bytes(), true);
    }

    #[test]
    fn extend() -> () {
        let mut s1:StdString = StdString::from("ab");
        s1.extend('c');
        assert_eq!(s1.as_bytes() == "abc".as_bytes(), true);
    }

    #[test]
    fn extend_chars() -> () {
        let mut s1:StdString = StdString::from("abc");
        s1.extend_chars(vec!['d', 'e', 'f']);
        assert_eq!(s1.as_bytes() == "abcdef".as_bytes(), true);
    }
   
    #[test]
    fn extend_str() -> () {
        let mut s1:StdString = StdString::from("Hello ");
        s1.extend_str("world!");
        assert_eq!(s1.as_bytes() == "Hello world!".as_bytes(), true);
    }
   
    #[test]
    fn length() -> () {
        let s1:StdString = StdString::from("Hello");
        assert_eq!(s1.len() == 5, true);
    }
   
    #[test]
    fn slice() -> () {
        let s1:StdString = StdString::from("Hello world!");
        assert_eq!(s1[11] == '!', true);
    }

    #[test]
    fn slice_range() -> () {
        let s1:StdString = StdString::from("Hello world!");
        assert_eq!(&s1[6..11] == "world", true);
    }

    #[test]
    fn partial_eq() -> () {
        let s1:StdString = StdString::from("TestString");
        let s2:StdString = StdString::from("OtherString");
        let s3:StdString = StdString::from("TestString");

        assert_eq!(s1 == s3, true);
        assert_eq!(s1 == s2, false);
        assert_eq!(s2 == s3, false);
    }
}
