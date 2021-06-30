extern crate bit_vec;
extern crate num_traits;

mod huffman {
    #![allow(dead_code)]
    use core::fmt;
    use std::str;
    
    use num::ToPrimitive;

    #[derive(PartialEq)]
    pub struct Node {
        character: char,
        count: usize
    }

    pub const EMPTY_NODE: Node = Node {character: '\0', count: 0}; // Empty Node with Char NULL and no count

    // This makes it possible to print out Nodes
    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(Char: '{}', Count: {})", self.character, self.count)
        }
    }

    pub fn get_nodes(s: &str) -> Vec<Node> {
        let mut res: Vec<Node> = Vec::<Node>::new(); // Result
        let mut checked: Vec<char> = Vec::new(); // Already checked characters
        let mut i: usize; // Matches of each iteration
        for c in s.chars() {
            if !checked.contains(&c){ // Check if already checked chars do not contain the current char
                i = s.matches(c).count().to_usize().unwrap(); // Get the total number of chars
                res.push(Node {character: c, count: i})  // Add a Node to the Result
            }
            checked.push(c) // Add the character to the checked chars
        }
        res.sort_by_key(|n| std::cmp::Reverse(n.count)); // Sort descending
        res // Return the Result
    }
}