#![allow(dead_code)]

pub mod lzw;
pub mod bypass;

pub trait Compressor {
    type Compressed;

    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String>;
    fn decompress(&mut self, data: Self::Compressed) -> Result<Vec<u8>, String>;
}
