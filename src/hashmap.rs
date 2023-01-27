/* Global allowances */
#![allow(dead_code, unused_variables)]

/* Imports */
use std::{ hash::{ Hash, Hasher }, collections::hash_map::DefaultHasher, fmt::Debug };

/* Structs */
pub struct HashMap<K: Hash, V> {
    pub items: Vec<Option<Node<K, V>>>,
}
#[derive(Debug)]
pub struct Node<K: Hash, V> {
    next: Option<Box<Self>>,
    value: V,
    key: K,
}

/* Method implementations */
impl<K: Hash, V> HashMap<K, V> {
    /* Constructor */
    pub fn new() -> Self {
        let mut items = Vec::with_capacity(10);
        for i in 0..10 { items.push(None); }

        Self { items }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        let mut items = Vec::with_capacity(capacity);
        for i in 0..capacity { items.push(None); }

        Self { items }
    }

    /* Getters */
    fn new_hasher() -> DefaultHasher {
        DefaultHasher::new()
    }
    fn get_index(&self, hash:u64) -> usize {
        let self_len = self.items.len();

        /* Get index */
        let index = hash % (self_len as u64).max(1);

        index as usize
    }
    fn get_hash(&self, k:&K) -> u64 {
        let mut hasher = Self::new_hasher();
        k.hash(&mut hasher);
        hasher.finish()
    }

    /* Insert item */
    pub fn insert(&mut self, k:K, v:V) -> () {
        let hash = self.get_hash(&k);
        let index = self.get_index(hash);

        /* Check availability */
        match self.items.get(index) {
            Some(e) => {
                match Self::last_next_element(match self.items.get_mut(index) {
                    Some(e) => e.as_mut(),
                    None => { self.items.splice(index..index+1, vec![Some(Node { next: None, value: v, key: k })]); return }
                }) {
                    Some(e) => e.next = Some(Box::new(Node { next: None, value: v, key: k })),
                    None => { self.items.splice(index..index+1, vec![Some(Node { next: None, value: v, key: k })]); return }
                }
            },
            None => { self.items.splice(index..index+1, vec![Some(Node { next: None, value: v, key: k })]); return }
        };
    }
    fn last_next_element(node: Option<&mut Node< K, V>>) -> Option<&mut Node< K, V>> {
        match node {
            Some(e) => {
                if e.next.is_some() {
                    Self::last_next_element(e.next.as_mut().map(|e| e.as_mut()))
                }else {
                    Some(e)
                }
            },
            None => None
        }
    }

    /* Get item */
    pub fn get(&self, k:K) -> Option<&V> {
        let hash = self.get_hash(&k);
        let index = self.get_index(hash);

        /* Get item */
        self.check_item(hash, match self.items.get(index) {
            Some(e) => e.as_ref(),
            None => return None
        })
    }
    fn check_item<'a>(&'a self, hash:u64, item:Option<&'a Node<K, V>>) -> Option<&'a V> {
        match &item {
            Some(e) => {
                let this_hash = &self.get_hash(&e.key);
                if this_hash == &hash {
                    return Some(&e.value)
                }else {
                    return self.check_item(
                        hash,
                        e.next.as_ref().map(|e| e.as_ref())
                    );
                }
            },
            None => None
        }
    }
}

/* Into key value tuple */
impl<'a, K: Hash, V> Into<(&'a K, &'a V)> for &'a Node<K, V> {
    fn into(self) -> (&'a K, &'a V) {
        (&self.key, &self.value)
    }
}

/* Debug implementations */
impl<K:Debug + Hash, V:Debug> std::fmt::Debug for HashMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes:Vec<(&K, &V)> = Vec::new();

        for item in self.items.iter() {
            nodes = vec![nodes, recurse(item.as_ref())].concat();
        };

        println!("HashMap {{");
        for (k, v) in nodes.iter() {
            println!("    {k:?}: {v:?},");
        };
        println!("}}");

        std::fmt::Result::Ok(())
    }
}
fn recurse<'a, K: Hash, V>(next:Option<&'a Node<K, V>>) -> Vec<(&'a K, &'a V)> {
    let mut nodes:Vec<(&K, &V)> = Vec::new();
    match next {
        Some(e) => {
            nodes = vec![
                nodes,
                vec![e.into()],
                recurse(
                    e.next
                        .as_ref()
                        .and_then(|e| Some(
                            e.as_ref()
                        )
                    )
                )
            ].concat();
        },
        None => ()
    };

    nodes
}
