use std::io::Write;
use std::path::Path;

use crate::chunk::{self, Chunk};

#[derive(Debug, thiserror::Error)]
pub enum EncodingError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid chunk name {0:?}")]
    Chunk(#[from] chunk::Error),
}

/// PNG encoder, including custom chunks
pub struct Encoder<W: Write> {
    writer: W,
    custom_chunks: Vec<Chunk>,
}

impl<W: Write> Encoder<W> {
    /// Create a neww encoder, adding the custom chunks before the image data.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            custom_chunks: vec![],
        }
    }
    /// Add a custom chunk, which will be placed before the image data.
    pub fn with_custom_chunk(mut self, chunk: Chunk) -> Self {
        self.custom_chunks.push(chunk);
        self
    }
}
impl Encoder<std::io::BufWriter<std::fs::File>> {
    /// Create a new encoder writing to a file
    pub fn new_to_file(output: impl AsRef<Path>) -> Result<Self, EncodingError> {
        let output = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output)?;
        let output = std::io::BufWriter::new(output);
        Ok(Self::new(output))
    }
}

fn encoding_error(err: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> image::ImageError {
    image::ImageError::Encoding(image::error::EncodingError::new(
        image::ImageFormat::Png.into(),
        err,
    ))
}

// This is similar to the implementation for image::codecs::png::PngEncoder
impl<W: Write> image::ImageEncoder for Encoder<W> {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        let (ct, bits) = match color_type {
            image::ExtendedColorType::L8 => (png::ColorType::Grayscale, png::BitDepth::Eight),
            image::ExtendedColorType::L16 => (png::ColorType::Grayscale, png::BitDepth::Sixteen),
            image::ExtendedColorType::La8 => (png::ColorType::GrayscaleAlpha, png::BitDepth::Eight),
            image::ExtendedColorType::La16 => {
                (png::ColorType::GrayscaleAlpha, png::BitDepth::Sixteen)
            }
            image::ExtendedColorType::Rgb8 => (png::ColorType::Rgb, png::BitDepth::Eight),
            image::ExtendedColorType::Rgb16 => (png::ColorType::Rgb, png::BitDepth::Sixteen),
            image::ExtendedColorType::Rgba8 => (png::ColorType::Rgba, png::BitDepth::Eight),
            image::ExtendedColorType::Rgba16 => (png::ColorType::Rgba, png::BitDepth::Sixteen),
            _ => {
                return Err(image::ImageError::Unsupported(
                    image::error::UnsupportedError::from_format_and_kind(
                        image::ImageFormat::Png.into(),
                        image::error::UnsupportedErrorKind::Color(color_type),
                    ),
                ))
            }
        };
        let mut encoder = png::Encoder::new(self.writer, width, height);
        encoder.set_color(ct);
        encoder.set_depth(bits);
        encoder.set_compression(png::Compression::Default);
        encoder.set_filter(png::FilterType::Sub);
        encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);
        let mut writer = encoder.write_header().map_err(encoding_error)?;
        for chunk in self.custom_chunks {
            writer
                .write_chunk(chunk.chunk_type.into(), &chunk.data)
                .map_err(encoding_error)?
        }
        writer.write_image_data(buf).map_err(encoding_error)?;
        writer.finish().map_err(encoding_error)?;
        Ok(())
    }
}
