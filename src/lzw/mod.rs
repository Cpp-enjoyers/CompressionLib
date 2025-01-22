#![allow(dead_code)]

use std::collections::HashMap;
use crate::Compressor;
use std::u16::MAX;
use std::cmp::Ordering;

#[cfg(test)]
mod test;

struct LZWCompressor{}

impl LZWCompressor {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Compressor for LZWCompressor {
    type Compressed = Vec<u16>;

    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String> {
        let reset = || -> HashMap<(u16, u8), u16>{
            (0u8..=255)
                .map(|i: u8| ((MAX, i), i as u16))
                .collect()
        };

        let mut dict: HashMap<(u16, u8), u16> = reset();
        let mut i: u16 = MAX;
        let mut out: Vec<u16> = Vec::new();
        
        for byte in data {
            if dict.len() == MAX as usize {
                dict = reset();
            }

            if !dict.contains_key(&(i, byte)) {
                dict.insert((i, byte), dict.len() as u16);
                out.push(i);
                i = dict[&(MAX, byte)];
            } else {
                i = dict[&(i, byte)];
            }
        }

        if i != MAX {
            out.push(i);
        }

        Ok(out)
    }

    fn decompress(&mut self, data: Self::Compressed) -> Result<Vec<u8>, String> {
        let reset = || -> Vec<(u16, u8)> {
            (0u8..=255)
            .map(|i: u8| (MAX, i))
            .collect()
        };

        // borrow per-call to avoid reborrow
        let rebuild_str = |dict: &Vec<(u16, u8)>, mut i: u16| -> Vec<u8> {
            let mut str: Vec<u8> = Vec::new();

            while i != MAX {
                let tmp: &(u16, u8) = &dict[i as usize];
                str.push(tmp.1);
                i = tmp.0;
            }

            str.reverse();
            str
        };

        let mut dict: Vec<(u16, u8)> = reset();
        let mut i: u16 = MAX;
        let mut str: Vec<u8>;
        let mut out: Vec<u8> = Vec::new();

        for j in data {
            if dict.len() == MAX as usize {
                dict = reset();
            }

            match j.cmp(&(dict.len() as u16)) {
                Ordering::Greater => {
                    return Err("invalid compressed code".to_string())
                },
                Ordering::Equal => {
                    // Intentional unwrap
                    dict.push((i, *rebuild_str(&dict, i).first().unwrap()));
                    str = rebuild_str(&dict, j);
                },
                Ordering::Less => {
                    str = rebuild_str(&dict, j);
                    if i != MAX {
                        // Intentional unwrap
                        dict.push((i, *str.first().unwrap()));
                    }
                },
            }

            out.append(&mut str);
            i = j;
        }

        Ok(out)
    }
}
