use std::collections::{BinaryHeap, HashMap};

use bit_vec::BitVec;
use itertools::Itertools;

use crate::Compressor;

#[cfg(test)]
mod test;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct HuffmanNode {
    freq: usize,
    c: Option<u8>, // breaks tie in frequencies
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(freq: usize, c: Option<u8>) -> Self {
        Self {
            freq,
            c,
            left: None,
            right: None,
        }
    }

    fn new_with_children(
        freq: usize,
        c: Option<u8>,
        left: Option<Box<HuffmanNode>>,
        right: Option<Box<HuffmanNode>>,
    ) -> Self {
        Self {
            freq,
            c,
            left,
            right,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct HuffmanCompressor {}

impl HuffmanCompressor {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Compressor for HuffmanCompressor {
    type Compressed = (BitVec, HashMap<u8, BitVec>);

    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String> {
        // this is a fn object, it has no real reason to be here besides being the only way
        // to create a recoursive clousure. I just wanted to try it out :c
        fn gen_codes(node: &HuffmanNode, prefix: &mut BitVec, codes: &mut HashMap<u8, BitVec>) {
            if let Some(character) = node.c {
                codes.insert(character, prefix.clone());
            } else {
                let mut clone: BitVec = prefix.clone();
                if let Some(ref left) = node.left {
                    prefix.push(false);
                    gen_codes(left, prefix, codes);
                }
                if let Some(ref right) = node.right {
                    clone.push(true);
                    gen_codes(right, &mut clone, codes);
                }
            }
        }

        let input: Vec<u8> = data.clone();
        let freqs: HashMap<u8, usize> = data.into_iter().counts();
        let mut heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();

        for (c, freq) in freqs {
            heap.push(HuffmanNode::new(freq, Some(c)));
        }

        while heap.len() > 1 {
            let left: HuffmanNode = heap.pop().unwrap();
            let right: HuffmanNode = heap.pop().unwrap();

            let sum: usize = left.freq + right.freq;
            let new_node: HuffmanNode = HuffmanNode::new_with_children(
                sum,
                None,
                Some(Box::new(left)),
                Some(Box::new(right)),
            );

            heap.push(new_node);
        }

        let root: Box<HuffmanNode> = heap
            .pop()
            .map_or(Box::new(HuffmanNode::new(0, None)), Box::new);
        let mut codes: HashMap<u8, BitVec> = HashMap::new();
        let mut start: BitVec = BitVec::new();
        // handle corner case of single character
        if root.c.is_some() {
            start.push(false);
        }
        gen_codes(&root, &mut start, &mut codes);

        let mut encoded: BitVec = BitVec::new();
        input
            .iter()
            .for_each(|b: &u8| encoded.append(&mut codes.get_mut(b).unwrap().clone()));

        Ok((encoded, codes))
    }

    fn decompress(&mut self, (encoded, map): Self::Compressed) -> Result<Vec<u8>, String> {
        // Create a reverse mapping from code to u8
        let reverse_codes: HashMap<BitVec, u8> = map.into_iter().map(|(k, v)| (v, k)).collect();

        let mut current_code: BitVec = BitVec::new();
        let mut decoded: Vec<u8> = Vec::new();

        for bit in &encoded {
            current_code.push(bit);
            if let Some(&b) = reverse_codes.get(&current_code) {
                decoded.push(b);
                current_code = BitVec::new();
            }
        }

        if current_code.is_empty() {
            Ok(decoded)
        } else {
            Err("Invalid compression code".to_owned())
        }
    }
}
