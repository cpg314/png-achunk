This library allows encoding and decoding custom chunks in PNG images.

These chunks can contain arbitrary binary data (of length at most 2^32-1), placed before or after image data.

If they are properly marked as ancillary (see [PNG specification, section 11.3](https://www.w3.org/TR/png/#11Ancillary-chunks)), good decoders will simply ignore them.

Currently, the encoder always places the custom chunks before the image data, which allows reading them without even decoding the image.

## Usage

### Rust

The [`Encoder`] struct implements the [`image::ImageEncoder`] trait, so that it can be used to encode [`image::DynamicImage`] images:

```rust
// Create custom chunk
// The name implies that this is a private non-critical chunk that is safe to copy
// only when non-critical chuks have not been modified.
let chunk_type = png_achunk::ChunkType::from_ascii(&"teST")?;
let chunk = png_achunk::Chunk::new(chunk_type, vec![4, 5, 6])?;
// Encode an `image::DynamicImage`
image.write_with_encoder(
    png_achunk::Encoder::new_to_file("test.png")?.with_custom_chunk(chunk.clone()),
)?;
```

For decoding:

```rust
// Decode image and ancillary chunks
let (image, chunks) = png_achunk::Decoder::from_file("test.png")?.decode_all()?;
// Decode ancillary chunks without reading the image data
let chunks = png_achunk::Decoder::from_file("test.png")?.decode_ancillary_chunks()?;
```

### Python

Only a primitive interface is implemented at the moment.

First, copy `target/release/libpng_achunk_py.so` to `png_achunk.so`.

```python
import png_achunk
chunk = png_achunk.read_chunk("test.png", "teST")
```

### C

Only a primitive interface is implemented at the moment.

The example in `c/test.c` can be compiled with

```bash
$ gcc test.c -L. -l:libpng_achunk.a -lm -o test
```

## Implementation

PNG encoding and decoding use the [`png`] crate. For decoding, we need to decode ancillary chunks manually as [`png::Decoder`] does not expose them.
