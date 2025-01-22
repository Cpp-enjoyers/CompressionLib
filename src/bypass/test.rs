#[cfg(test)]
mod tests {
    use crate::{bypass::BypassCompressor, Compressor};

    #[test]
    fn test1() {
        let mut bypass: BypassCompressor = BypassCompressor::new();
        let compressed: Result<Vec<u8>, String> = bypass.compress(Vec::from("data".as_bytes()));
        assert!(compressed.is_ok());
        let compressed = compressed.unwrap();
        let decompressed: Result<Vec<u8>, String> = bypass.decompress(compressed);
        assert!(decompressed.is_ok());
        assert_eq!(decompressed.unwrap(), Vec::from("data".as_bytes()));
    }
}