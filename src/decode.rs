use std::io::{BufRead, BufReader, Seek};
use std::path::Path;

use crate::chunk::{self, Chunk};

#[derive(Debug, thiserror::Error)]
pub enum DecodingError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not a PNG")]
    NotPng,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Failed to decode image")]
    Image(#[from] image::ImageError),
    #[error("Invalid chunk name {0:?}")]
    Chunk(#[from] chunk::Error),
    #[error("Invalid filename")]
    Filename(#[from] std::str::Utf8Error),
    #[error("Chunk not found")]
    ChunkNotFound,
}

/// PNG decoding, including ancillary chunks.
pub struct Decoder<R: BufRead + Seek> {
    reader: R,
}
impl<R: BufRead + Seek> Decoder<R> {
    pub fn from_reader(reader: R) -> Self {
        Self { reader }
    }
}
impl Decoder<BufReader<std::fs::File>> {
    /// Create a decoder from a file, using buffered reading.
    pub fn from_file(filename: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let reader = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(reader);
        Ok(Self::from_reader(reader))
    }
}
impl<R: BufRead + Seek> Decoder<R> {
    /// Decode the non-critical chunks only, without reading the full image if they are placed before
    /// the IDAT section.
    pub fn decode_ancillary_chunks(&mut self) -> Result<Vec<Chunk>, DecodingError> {
        self.reader.seek(std::io::SeekFrom::Start(0))?;
        // Decode chunks
        let mut header = [0; 8];
        self.reader.read_exact(&mut header)?;
        if &header[1..4] != b"PNG" {
            return Err(DecodingError::NotPng);
        }
        let mut chunks = vec![];
        loop {
            let c = Chunk::from_reader(&mut self.reader)?;
            if c.chunk_type == png::chunk::IEND || c.chunk_type == png::chunk::IDAT {
                break;
            }
            if !c.chunk_type.is_critical() {
                chunks.push(c);
            }
        }
        Ok(chunks)
    }
    /// Decode image and ancillary chunks.
    pub fn decode_all(&mut self) -> Result<(image::DynamicImage, Vec<Chunk>), DecodingError> {
        let chunks = self.decode_ancillary_chunks()?;
        // Decode image
        // TODO: Avoid re-reading the ancillary chunks
        self.reader.seek(std::io::SeekFrom::Start(0))?;
        let image = image::DynamicImage::from_decoder(image::codecs::png::PngDecoder::new(
            &mut self.reader,
        )?)?;
        Ok((image, chunks))
    }
}
