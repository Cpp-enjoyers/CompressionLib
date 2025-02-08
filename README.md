 `CppEnjoyers` compression library used by WebClient and WebServer to compress
  the packets sent on the network. The library exposes the trait `Compressor`
  that offers a generic minimal API for compression so a user can implement their
  own compression algorithms and easily intergate it with this library
 
  By default the following compressors are available:
  - `HuffmanCompressor` -> uses Huffman compression
  - `LZWCompressor` -> uses LZW compression, very fast but small gains
  - `BypassCompressor` -> this compressor just return the input unchanged,
       it is used to logically represent that no compression happens
 
  # Example usage
  ``` rust
  use compression::LZWCompressor;
  use crate::compression::Compressor;
  fn main() {
      let data: Vec<u8> = vec!1, 2, 3, 4, 5;
      let mut compressor: LZWCompressor = LZWCompressor::new();
      let compressed = compressor.compress(data.clone());
      assert!(compressed.is_ok());
      assert!(data == compressor.decompress(compressed.unwrap()).unwrap())
  }
