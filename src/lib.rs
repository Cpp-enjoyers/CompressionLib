#![allow(dead_code)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
// #![deny(missing_docs)]

pub mod bypass;
pub mod huffman;
pub mod lzw;
#[cfg(test)]
mod test_utils;

pub trait Compressor {
    type Compressed;

    /// # Errors
    ///
    /// Return Err if data is not compressable
    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String>;

    /// # Errors
    ///
    /// Return Err if data is ill formed (i.e. uncompressable)
    fn decompress(&mut self, data: Self::Compressed) -> Result<Vec<u8>, String>;
}
