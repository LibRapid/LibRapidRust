#![allow(dead_code)]
extern crate bit_vec;
extern crate num_traits;
use core::fmt;
use std::{str, usize};
use std::collections::HashMap;

use bit_vec::BitVec;

type Link = Option<Box<Node>>;

#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub character: Option<char>,
    pub frequency: usize,
    pub left: Link,
    pub right: Link
}

// This makes it possible to print Nodes
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.character {
            None => write!(f, "Count: {})", self.frequency),
            _ => write!(f, "(Char: '{}', Count: {})", self.character.unwrap(), self.frequency),
        }
    }
}

pub fn new_node(freq: usize, c: Option<char>) -> Node {
    Node {
        frequency: freq, character: c,
        left: None, right: None,
    }
}

pub fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

pub fn get_frequency(s: &str) -> HashMap<char, usize> {
    let mut hm = HashMap::new(); // Result
    for c in s.chars() {
        let counter = hm.entry(c).or_insert(0); // Inserts c if value is not present, returns mut ref
        *counter += 1; // Increment by 1
    }
    hm
}



// !! CRITICAL BUG !!
pub fn assign_codes(root: &Box<Node>, 
                    hashmap: &mut HashMap<char, BitVec>, 
                    bitvec: &mut BitVec ) {
    match root.character {
        Some(character) => { hashmap.insert(character, bitvec.clone()); }

        None => { 
            if let Some(ref l) = root.left { bitvec.push(false); assign_codes(l, hashmap, bitvec); }
            if let Some(ref r) = root.right { bitvec.push(true); assign_codes(r, hashmap, bitvec); }
        }
    }
}
// !! END CRITICAL BUG !!



pub fn encode_string(s: &str, hashmap: &mut HashMap<char, BitVec>) -> BitVec {
    let mut res: BitVec = BitVec::new();
    let mut t: Option<&mut BitVec>;

    for c in s.chars() {
        t = hashmap.get_mut(&c);
        res.append(t.unwrap());
    }
    res
}

pub fn decode_string(bitvec: &BitVec, root: &Box<Node>) -> String {
    let mut res = "".to_string();
    let mut nodeptr = root;

    for b in bitvec {
        if b == false {
            if let Some(ref l) = nodeptr.left { nodeptr = l; }
        } else {
            if let Some (ref r) = nodeptr.right { nodeptr = r; }
        }
        if let Some(c) = nodeptr.character {
            res.push(c);
            nodeptr = root;
        }
    }

    res
}

pub fn get_nodes(s: &str) -> Vec<Box<Node>> {
    let f = get_frequency(s);
    let mut res: Vec<Box<Node>> = f.iter()
                                  .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
                                  .collect(); // Result
    res.sort_by_key(|n| n.frequency); // Sort ascending after count
    res // Return the Result
}

pub fn get_tree(mut p: Vec<Box<Node>>) -> Vec<Box<Node>> {
    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.frequency)).cmp(&(a.frequency))); // Sort
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.frequency + b.frequency, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }
    p
}

pub fn get_root(mut p: Vec<Box<Node>>) -> Box<Node> {
    p.pop().unwrap()
}