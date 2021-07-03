#![allow(dead_code)]
extern crate bit_vec;
extern crate num_traits;
extern crate bincode;
extern crate serde;
extern crate plain;

use plain::{Plain};
use core::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::{str, usize};
use std::collections::HashMap;
use bit_vec::BitVec;
use num::ToPrimitive;
use serde::{Serialize, Deserialize};
use std::convert::TryInto;

type Link = Option<Box<Node>>;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Node {
    pub character: Option<char>,
    pub frequency: usize,
    pub left: Link,
    pub right: Link
}

unsafe impl Plain for Node {}

impl Node {
    fn from_bytes(buf: &[u8]) -> &Node {
        plain::from_bytes(buf).expect("The buffer is either too short or not aligned!")
    }

    fn from_mut_bytes(buf: &mut [u8]) -> &mut Node {
        plain::from_mut_bytes(buf).expect("The buffer is either too short or not aligned!")
    }

    fn copy_from_bytes(buf: &[u8]) -> Node {
        let mut h = Node::default();
        h.copy_from_bytes(buf).expect("The buffer is too short!");
        h
    }
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

fn new_node(freq: usize, c: Option<char>) -> Node {
    Node {
        frequency: freq, character: c,
        left: None, right: None,
    }
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

fn get_frequency(s: &str) -> HashMap<char, usize> {
    let mut hm = HashMap::new(); // Result
    for c in s.chars() {
        let counter = hm.entry(c).or_insert(0); // Inserts c if value is not present, returns mut ref
        *counter += 1; // Increment by 1
    }
    hm
}

pub fn assign_codes(root: &Box<Node>, 
                    hashmap: &mut HashMap<char, BitVec>, 
                    bitvec: &mut BitVec ) {
    match root.character {
        Some(character) => { hashmap.insert(character, bitvec.clone()); }

        None => {
            if let Some(ref l) = root.left
            { bitvec.push(false); assign_codes(l, hashmap, bitvec); }

            if let Some(ref r) = root.right
            { bitvec.push(true); assign_codes(r, hashmap, bitvec); }
        }
    }
    bitvec.pop();
    // Thanks to Pencilcaseman to fixing a issue
}

pub fn huffman_encode(s: &str, char_codes: &mut HashMap<char, BitVec>) -> BitVec {
    let mut res: BitVec = BitVec::new();
    let mut t: Option<&mut BitVec>;

    for c in s.chars() {
        t = char_codes.get_mut(&c);
        res.append(&mut t.cloned().unwrap());
    }
    res
}

fn decode_string(bitvec: &BitVec, root: &Box<Node>) -> String {
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

pub fn huffman_decode(bitvec: &BitVec, root: &Box<Node>) -> String {
    decode_string(bitvec, root)
}

fn get_p(s: &str) -> Vec<Box<Node>> {
    let frequency = get_frequency(s);
    let mut p: Vec<Box<Node>> = frequency.iter().map(|x| new_box(new_node(*(x.1), Some(*(x.0))))).collect();

    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.frequency)).cmp(&(a.frequency)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.frequency + b.frequency, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }

    p
}

pub fn get_root(s: &str) -> Box<Node> {
    let frequency = get_frequency(s);
    let mut p: Vec<Box<Node>> = frequency.iter().map(|x| new_box(new_node(*(x.1), Some(*(x.0))))).collect();

    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.frequency)).cmp(&(a.frequency)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.frequency + b.frequency, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }
    p.pop().unwrap()
}

pub fn write_to_file(mut path: String, bitvec: &BitVec, root: &Box<Node>) {
    path = path + ".hlr";
    let len: u64 = bitvec.to_bytes().len().to_u64().unwrap();
    let mut file = File::create(path).unwrap();
    let _wrlen = file.write(&len.to_ne_bytes());
    println!("BitVec to bytes: {:?}", &bitvec.to_bytes());
    println!("BitVec from bytes: {:?}", BitVec::from_bytes(&bitvec.to_bytes()));
    let _wrbitvec = file.write(&bitvec.to_bytes());
    let _wrroot = bincode::serialize_into(&file, root);
    println!("root to bytes: {:?}", _wrroot.unwrap());
}

pub fn read_from_file(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = Vec::<u8>::new();
    
    let _ = file.read_to_end(&mut buf);
    let mut _len = buf.clone();
    let mut _bitvec = buf.clone();
    let mut _root = buf.clone();
    _bitvec.drain(0..7);
    println!("Vec<u8>: {:?}", _len);
    _len.truncate(8);
    let len = u64::from_ne_bytes(_len.try_into().unwrap());
    _bitvec.drain(len.to_usize().unwrap()+1.to_usize().unwrap().._bitvec.len());
    let mut _bitvec2 = BitVec::from_bytes(&_bitvec);
    let bitvec = _bitvec2.split_off(8);
    println!("Decoded BitVec: {:?}", &bitvec);
    println!("Decoded Length: {}", &len);
    _root.drain(0..7+bitvec.len());
    let root = bincode::deserialize(&_root).unwrap(); // called `Result::unwrap()` on an `Err` value: InvalidTagEncoding()
    println!("{:?}", root);
    huffman_decode(&bitvec, &root)
}