#![allow(dead_code)]

use crate::Compressor;

#[cfg(test)]
mod test;

pub struct BypassCompressor{}

impl BypassCompressor {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Compressor for BypassCompressor {
    type Compressed = Vec<u8>;

    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String> {
        Ok(data)
    }

    fn decompress(&mut self, data: Self::Compressed) -> Result<Vec<u8>, String> {
        Ok(data)
    }
}
