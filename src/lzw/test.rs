#[cfg(test)]
mod tests {
    use crate::lzw::LZWCompressor;
    use crate::test_utils::{
        generic_test1, generic_test2, generic_test3, generic_test4, generic_test5,
    };
    use crate::Compressor;

    #[test]
    fn test1() {
        generic_test1(LZWCompressor::new());
    }

    #[test]
    fn test2() {
        generic_test2(LZWCompressor::new());
    }

    #[test]
    fn test3() {
        generic_test3(LZWCompressor::new());
    }

    #[test]
    fn test4() {
        generic_test4(LZWCompressor::new());
    }

    #[test]
    fn test5() {
        generic_test5(LZWCompressor::new());
    }

    #[test]
    fn test_fail() {
        let mut c: Vec<u16> = LZWCompressor::new()
            .compress(Vec::from("ciao".as_bytes()))
            .unwrap();
        c[0] = 3454;
        let decompress: Result<Vec<u8>, String> = LZWCompressor::new().decompress(c);
        assert!(decompress.is_err());
    }
}
