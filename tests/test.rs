use image::DynamicImage;

#[test]
fn test() -> anyhow::Result<()> {
    let image = DynamicImage::from(image::RgbImage::from_pixel(
        10,
        20,
        image::Rgb([10, 11, 12]),
    ));

    let chunk_type = png_achunk::ChunkType::from_ascii(&"teST")?;
    assert!(!chunk_type.is_critical());
    assert!(!chunk_type.is_public());
    assert!(!chunk_type.is_safe_to_copy());
    let chunk = png_achunk::Chunk::new(chunk_type, vec![4, 5, 6])?;

    // Create the file ourselves
    let tempdir = tempfile::tempdir()?;
    let filename = tempdir.path().join("out.png");

    image.write_with_encoder(
        png_achunk::Encoder::new_to_file(&filename)?.with_custom_chunk(chunk.clone()),
    )?;

    std::fs::copy(&filename, "/tmp/test.png")?;

    // Internal decoding
    let (decoded_image, chunks) = png_achunk::Decoder::from_file(&filename)?.decode_all()?;
    assert_eq!(image, decoded_image);
    assert_eq!(vec![chunk], chunks);

    // External decoding, ignoring extra chunks
    let decoded_image = image::open(filename)?;
    assert_eq!(image, decoded_image);

    Ok(())
}
