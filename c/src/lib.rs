fn read_chunk_impl(
    filename: &std::ffi::CStr,
    name: &std::ffi::CStr,
) -> Result<Vec<u8>, png_achunk::decode::DecodingError> {
    let chunks = png_achunk::Decoder::from_file(filename.to_str()?)?.decode_ancillary_chunks()?;
    let chunk_type = png_achunk::ChunkType::from_ascii(&name)?;
    let chunk = chunks
        .into_iter()
        .find(|c| c.chunk_type == chunk_type)
        .ok_or_else(|| png_achunk::decode::DecodingError::ChunkNotFound)?;
    Ok(chunk.data)
}

#[no_mangle]
unsafe extern "C" fn read_chunk(
    filename: *const std::ffi::c_char,
    name: *const std::ffi::c_char,
    ptr: *mut *mut u8,
    len: *mut libc::size_t,
) {
    let filename = std::ffi::CStr::from_ptr(filename);
    let name = std::ffi::CStr::from_ptr(name);
    let mut data = match read_chunk_impl(filename, name) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            *ptr = std::ptr::null_mut();
            return;
        }
    };

    data.shrink_to_fit();
    assert!(data.len() == data.capacity());
    *ptr = data.as_mut_ptr();
    *len = data.len();
    std::mem::forget(data);
}

#[no_mangle]
unsafe extern "C" fn free_chunk(ptr: *mut u8, len: *const libc::size_t) {
    let len = len as usize;
    drop(Vec::<u8>::from_raw_parts(ptr, len, len));
}
