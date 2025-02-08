/*!
 * `CppEnjoyers` compression library used by WebClient and WebServer to compress
 * the packets sent on the network. The library exposes the trait [`Compressor`]
 * that offers a generic minimal API for compression so a user can implement their
 * own compression algorithms and easily intergate it with this library
 *
 * By default the following compressors are available:
 * - [`HuffmanCompressor`] -> uses Huffman compression
 * - [`LZWCompressor`] -> uses LZW compression, very fast but small gains
 * - [`BypassCompressor`] -> this compressor just return the input unchanged,
 *      it is used to logically represent that no compression happens
 *
 * # Example usage
 * ```
 * # use compression::LZWCompressor;
 * # use crate::compression::Compressor;
 * # fn main() {
 * let data: Vec<u8> = vec![1, 2, 3, 4, 5];
 * let mut compressor: LZWCompressor = LZWCompressor::new();
 * let compressed = compressor.compress(data.clone());
 * assert!(compressed.is_ok());
 * assert!(data == compressor.decompress(compressed.unwrap()).unwrap())
 * # }
 * ```
 */

#![allow(dead_code)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![deny(nonstandard_style)]
#![warn(missing_docs)]

/// module implementing bypass compression
pub mod bypass;
/// module implementing Huffman compression
pub mod huffman;
/// module implementing LZW compression
pub mod lzw;
/// testing utilities module
#[cfg(test)]
mod test_utils;

#[doc(inline)]
pub use bypass::BypassCompressor;
#[doc(inline)]
pub use huffman::HuffmanCompressor;
#[doc(inline)]
pub use lzw::LZWCompressor;

/// Offers minimal API to implement a compressor object
pub trait Compressor {
    /// The type output of the compressor
    type Compressed;

    /// Compresses the given input
    /// # Errors
    ///
    /// Return Err if data is not compressable
    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String>;

    /// Decompresses the given input
    /// # Errors
    ///
    /// Return Err if data is ill formed (i.e. uncompressable)
    fn decompress(&mut self, data: Self::Compressed) -> Result<Vec<u8>, String>;
}
