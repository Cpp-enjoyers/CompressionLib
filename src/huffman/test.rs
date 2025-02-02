#[cfg(test)]
mod tests {
    use crate::{
        huffman::HuffmanCompressor,
        test_utils::{generic_test1, generic_test2, generic_test3, generic_test4, generic_test5},
        Compressor,
    };

    #[test]
    fn test1() {
        generic_test1(HuffmanCompressor::new());
    }

    #[test]
    fn test2() {
        generic_test2(HuffmanCompressor::new());
    }

    #[test]
    fn test3() {
        generic_test3(HuffmanCompressor::new());
    }

    #[test]
    fn test4() {
        generic_test4(HuffmanCompressor::new());
    }

    #[test]
    fn test5() {
        generic_test5(HuffmanCompressor::new());
    }

    #[test]
    fn test_fail() {
        let (mut c, m) = HuffmanCompressor::new()
            .compress(Vec::from("ciao".as_bytes()))
            .unwrap();
        c.pop();
        let decompress: Result<Vec<u8>, String> = HuffmanCompressor::new().decompress((c, m));
        assert!(decompress.is_err());
    }
}
