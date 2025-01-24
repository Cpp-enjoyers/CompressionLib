use crate::Compressor;

#[cfg(test)]
mod test;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct BypassCompressor {}

impl BypassCompressor {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Compressor for BypassCompressor {
    type Compressed = Vec<u8>;

    #[inline]
    fn compress(&mut self, data: Vec<u8>) -> Result<Self::Compressed, String> {
        Ok(data)
    }

    #[inline]
    fn decompress(&mut self, data: Self::Compressed) -> Result<Vec<u8>, String> {
        Ok(data)
    }
}
