extern crate bit_vec;
extern crate bincode;
extern crate serde;

use core::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::{str, usize};
use std::collections::HashMap;
use bit_vec::BitVec;
use serde::{Serialize, Deserialize};

use crate::math::general::NumTools;
/// Encoding trait for minimal code writing.
pub trait Encode {
    /// Fully encodes a String.
    ///
    /// # Returns
    /// A `(BitVec, Box<Node>)` tuple.
    ///
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::compression::huffman::{Node, Encode, Decode};
    ///
    /// let s: &str = "Lorem Ipsum";
    ///
    /// let enc = s.full_encode();
    /// let dec = enc.full_decode();
    /// 
    /// assert_eq!("Lorem Ipsum".to_owned(), dec);
    /// ```
    #[must_use]
    fn full_encode(&self) -> (BitVec, Box<Node>);
}
/// Decoding trait for minimal code writing.
pub trait Decode {
    /// Fully decodes a encoded String.
    ///
    /// # Returns
    /// A `String`.
    ///
    /// # Examples
    /// ```
    /// use lib_rapid::compsci::compression::huffman::{Node, Encode, Decode};
    ///
    /// let s: &str = "Lorem Ipsum";
    ///
    /// let enc = s.full_encode();
    /// let dec = enc.full_decode();
    /// 
    /// assert_eq!("Lorem Ipsum".to_owned(), dec);
    /// ```
    #[must_use]
    fn full_decode(&self) -> String;
}

impl Encode for String {
    fn full_encode(&self) -> (BitVec, Box<Node>) {
        let root = get_root(self);
        let mut char_codes: HashMap<char, BitVec> = HashMap::new();
        assign_codes(&root, &mut char_codes, &mut BitVec::new());
        (huffman_encode(self, &char_codes), root)
    }
}

impl Encode for &str {
    fn full_encode(&self) -> (BitVec, Box<Node>) {
        let root = get_root(self);
        let mut char_codes: HashMap<char, BitVec> = HashMap::new();
        assign_codes(&root, &mut char_codes, &mut BitVec::new());
        (huffman_encode(self, &char_codes), root)
    }
}

impl Decode for (BitVec, Box<Node>) {
    fn full_decode(&self) -> String {
        decode_string(&self.0, &self.1)
    }
}

type Link = Option<Box<Node>>;
/// The struct for a node.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Node {
    pub character: Option<char>,
    pub frequency: u128,
    pub left:      Link,
    pub right:     Link
}

impl Node {
    /// Generate a new node.
    fn new(freq: u128, c: Option<char>) -> Node {
        Node {
            frequency: freq,
            character: c,
            left:      None, 
            right:     None,
        }
    }
    
    /// Alias for `Box::new(n)`.
    fn new_box(n: Node) -> Box<Node> {
        Box::new(n)
    }
}

// This makes it possible to print Nodes
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.character {
            None => write!(f, "Count: {})", self.frequency),
            _    => write!(f, "(Char: '{}', Count: {})", self.character.unwrap(), self.frequency),
        }
    }
}
/// Get the frequency of the characters in a given String.
fn get_frequency(s: &str) -> HashMap<char, usize> {
    let mut hm: HashMap<char, usize> = HashMap::new(); // Result
    for c in s.chars() {
        let counter: &mut usize = hm.entry(c)
                                    .or_insert(0); // Inserts c if value is not present, returns mut ref
        counter.inc(); // Increment by 1
    }
    hm
}

/// Assigns Huffman-codes to each character.
///
/// # Arguments
/// * `root` - The Huffman tree.
/// * `hashmap` - The variable in which the result is stored.
/// * `bitvec` - A temporary BitVec.
///
/// # Returns
/// Nothing.
///
/// # Examples
/// ```
/// use lib_rapid::compsci::compression::huffman::{get_root, assign_codes};
/// use bit_vec::BitVec;
/// use std::collections::HashMap;
///
/// let s = "Lorem Ipsum";
/// let root = get_root(s);
/// let mut char_codes:HashMap<char, BitVec> = HashMap::new();
/// assign_codes(&root, &mut char_codes, &mut BitVec::new()); // Assigns codes to characters of s and stores them in char_codes.
/// ```
pub fn assign_codes(root: &Box<Node>, 
                    hashmap: &mut HashMap<char, BitVec>, 
                    bitvec:  &mut BitVec ) {
    match root.character {
        Some(character) => { hashmap.insert(character, bitvec.clone()); }

        None => {
            if let Some(ref l) = root.left
            { bitvec.push(false);
              assign_codes(l, hashmap, bitvec); }

            if let Some(ref r) = root.right
            { bitvec.push(true);
              assign_codes(r, hashmap, bitvec); }
        }
    }
    bitvec.pop();
    // Thanks to Pencilcaseman for fixing an issue
}

/// Encodes a string.
///
/// # Arguments
/// * `s` - The string to be encoded.
/// * `char_codes` - The assigned Huffman codes of the characters.
///
/// # Returns
/// A BitVec which contains the Huffman encoded string.
///
/// # Examples
/// ```
/// use lib_rapid::compsci::compression::huffman::{get_root, assign_codes, huffman_encode};
/// use bit_vec::BitVec;
/// use std::collections::HashMap;
///
/// let s = "Lorem Ipsum";
/// let root = get_root(s);
/// let mut char_codes:HashMap<char, BitVec> = HashMap::new();
/// assign_codes(&root, &mut char_codes, &mut BitVec::new()); // Assigns codes to characters of s and stores them in char_codes.
///
/// let enc = huffman_encode(s, &char_codes); // Encodes the String s into enc.
/// ```
#[must_use]
pub fn huffman_encode(s: &str, char_codes: &HashMap<char, BitVec>) -> BitVec {
    let mut res: BitVec = BitVec::with_capacity(s.len());
    let mut t:   Option<&BitVec>;

    for c in s.chars() {
        t = char_codes.get(&c);
        res.append(&mut t.cloned()
           .unwrap());
    }
    res
}
/// Decodes a String.
fn decode_string(bitvec: &BitVec, root: &Box<Node>) -> String {
    let mut res:     String     = String::new();
    let mut nodeptr: &Box<Node> = root;

    for b in bitvec {
        match b {
            false => { if let Some(ref l) = nodeptr.left { nodeptr = l; } }
            true  => { if let Some(ref r) = nodeptr.right { nodeptr = r; } }
        }

        if let Some(c) = nodeptr.character { res.push(c); nodeptr = root; }
    }

    res
}

/// Decodes a Huffman encoded BitVec.
///
/// # Arguments
/// * `bitvec` - The assigned Huffman codes of the characters.
/// * `root` - The Huffman tree.
///
/// # Returns
/// A BitVec which contains the Huffman encoded string.
///
/// # Examples
/// ```
/// use lib_rapid::compsci::compression::huffman::{get_root, assign_codes, huffman_encode, huffman_decode};
/// use bit_vec::BitVec;
/// use std::collections::HashMap;
///
/// let s: &str = "Lorem Ipsum";
/// let root = get_root(s);
/// let mut bitvec = BitVec::new();
/// let mut char_codes: HashMap<char, BitVec> = HashMap::new();
/// assign_codes(&root, &mut char_codes, &mut bitvec); // Assigns codes to characters of s and stores them in char_codes.
///
/// let enc = huffman_encode(s, &char_codes); // Encodes the String s into enc.
/// let dec = huffman_decode(&enc, &root); // Decodes the BitVec which was created by the last line.
/// 
/// assert_eq!("Lorem Ipsum".to_owned(), dec);
/// ```
#[must_use]
pub fn huffman_decode(bitvec: &BitVec, root: &Box<Node>) -> String {
    decode_string(bitvec, root)
}

/// Gets the Huffman tree for a string.
///
/// # Arguments
/// * `s` - The string of which the tree should be created.
///
/// # Returns
/// A Box<Node> containing the entire tree.
#[must_use]
pub fn get_root(s: &str) -> Box<Node> {
    let frequency = get_frequency(s);
    let mut vec_nodes: Vec<Box<Node>> = frequency.iter().map(|x| Node::new_box(Node::new(*(x.1) as u128, Some(*(x.0))))).collect();
    let mut a;
    let mut b;
    let mut c;

    while vec_nodes.len() > 1 {
        vec_nodes.sort_by(|a: &Box<Node>, b: &Box<Node>| (&(b.frequency)).cmp(&(a.frequency)));
        a = vec_nodes.pop().unwrap();
        b = vec_nodes.pop().unwrap();
        c = Node::new_box(Node::new( a.frequency + b.frequency, None));

        c.left  = Some(a);
        c.right = Some(b);
        vec_nodes.push(c);
    }
    vec_nodes.pop().unwrap()
}

/// Writes the encoded message into 2 files.
///
/// # Arguments
/// * `path` - The path to be written to.
/// * `bitvec` - The Huffman codes assigned to the characters.
/// * `root` - The Huffman tree.
///
/// # Returns
/// Nothing. Writes to 2 files:
/// name.hlr: The main file in which the encoded message is stored.
/// name.htlr: The file in which the huffman tree is stored.
///
/// # Examples
/// ```
/// use lib_rapid::compsci::compression::huffman::{Encode, write_to_file};///
/// let s: &str = "Lorem Ipsum";
/// 
/// let enc = s.full_encode(); // Encodes the String s into enc.
/// write_to_file("test".to_string(), &enc.0, &enc.1);
/// ```
pub fn write_to_file(path: String, bitvec: &BitVec, root: &Box<Node>) {
    let mut mainfile: File = File::create(path.clone() + ".hlr").unwrap();
    let mut treefile: File = File::create(path + ".htlr").unwrap();
    let _                  = mainfile.write(&bitvec.to_bytes());
    let _                  = treefile.write(&bincode::serialize(root).unwrap());
}

/// Reads and decodes the message stored in the .hlr and .htlr files.
///
/// # Arguments
/// * `path` - The path to be read from.
///
/// # Returns
/// A String containing the decoded message.
///
/// # Examples
/// ```
/// use lib_rapid::compsci::compression::huffman::{Encode, Decode, write_to_file, read_from_file};
/// let s: &str = "Lorem Ipsum123123123";
/// 
/// let enc = s.full_encode(); // Encodes the String s into enc.
/// let dec = enc.full_decode(); // Decodes the BitVec which was created by the last line.
/// write_to_file("test".to_string(), &enc.0, &enc.1);
/// 
/// let dec_written = read_from_file("test".to_string());
/// assert_eq!(dec_written, "Lorem Ipsum123123123");
/// ```
#[must_use]
pub fn read_from_file(path: String) -> String {
    let mut encoded_file: File    = File::open(path.clone() + ".hlr").unwrap();
    let mut tree_file:    File    = File::open(path + ".htlr").unwrap();
    let mut enc_file:     Vec<u8> = Vec::<u8>::new();
    let mut enc_tree:     Vec<u8> = Vec::<u8>::new();

    let _ = encoded_file.read_to_end(&mut enc_file);
    let _ = tree_file.read_to_end(&mut enc_tree);

    let mut bitvec: BitVec = BitVec::from_bytes(&enc_file); // Get final bitvec
    bitvec.pop();
    let root:Box<Node> = bincode::deserialize(&enc_tree).unwrap();

    huffman_decode(&bitvec, &root)
}