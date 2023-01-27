/* Imports */
use std::{
    fmt::{ Display, Debug },
    ops::{ Index, Range },
};

/* Struct */
pub struct StdString {
    inner: Vec<u8>
}

/* Method implementations */
impl StdString {
    /* Constructors */
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: Vec::with_capacity(capacity) }
    }

    /* Extend */
    pub fn extend(&mut self, char_:char) -> () {
        self.inner.push(char_ as u8)
    }
    pub fn extend_chars(&mut self, chars:Vec<char>) -> () {
        self.inner.extend(chars.iter().map(|e| *e as u8))
    }
    pub fn extend_str(&mut self, str_:&str) -> () {
        self.inner.extend(str_.chars().map(|e| e as u8))
    }

    /* As-casting */
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    /* Functions */
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/* Indexing */
impl Index<usize> for StdString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        let u = self.inner[index] as char;
        Box::leak(Box::new(u))
    }
}
impl Index<Range<usize>> for StdString {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        unsafe { std::str::from_utf8_unchecked(&self.inner[..][index]) }
    }
}

/* From and Into */
impl From<&str> for StdString {
    fn from(value: &str) -> Self {
        let mut end = Vec::with_capacity(value.len());
        for char_ in value.chars() {
            end.push(char_ as u8)
        };

        Self::from(end)
    }
}
impl From<Vec<u8>> for StdString {
    fn from(value: Vec<u8>) -> Self {
        Self { inner: value }
    }
}

/* Display */
impl Display for StdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.inner.iter().map(|e| *e as char).collect::<Vec<char>>();
        for char_ in chars { write!(f, "{char_}")?; };
        std::fmt::Result::Ok(())
    }
}
impl Debug for StdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.inner.iter().map(|e| *e as char).collect::<Vec<char>>();
        write!(f, "\"")?;
        for char_ in chars { write!(f, "{char_}")?; };
        write!(f, "\"")?;
        std::fmt::Result::Ok(())
    }
}

/* Compare */
impl PartialEq for StdString {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() { false }
        else {
            for (c1, c2) in self.inner.iter().zip(&other.inner) {
                if c1 != c2 { return false }
            }

            true
        }
    }
}

/* Clone impls */
impl Clone for StdString {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
    }
}

/* Default */
impl Default for StdString {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}
