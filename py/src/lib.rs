use std::io::{BufRead, Cursor, Seek};
use std::path::PathBuf;

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes, types::PyString};

/// Reads the ancillary chunk with the given `name` from the PNG `bytes`.
#[pyfunction]
fn read_chunk_from_png_bytes<'a>(
    py: Python<'a>,
    bytes: &'a PyBytes,
    name: &'a PyString,
) -> PyResult<&'a PyBytes> {
    let cursor = Cursor::new(bytes.as_bytes());
    let decoder = png_achunk::Decoder::from_reader(cursor);
    read_chunk_from_decoder(py, decoder, name)
}

/// Reads the ancillary chunk with the given `name` from the PNG file `filename`.
#[pyfunction]
fn read_chunk<'a>(
    py: Python<'a>,
    filename: &'a PyString,
    name: &'a PyString,
) -> PyResult<&'a PyBytes> {
    let decoder = png_achunk::Decoder::from_file(filename.extract::<PathBuf>()?)?;
    read_chunk_from_decoder(py, decoder, name)
}

fn read_chunk_from_decoder<'a, R: BufRead + Seek>(
    py: Python<'a>,
    mut decoder: png_achunk::Decoder<R>,
    name: &'a PyString,
) -> PyResult<&'a PyBytes> {
    let chunks = decoder
        .decode_ancillary_chunks()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to decode PNG chunks: {}", e)))?;
    let name: String = name.extract()?;
    let chunk_type = png_achunk::ChunkType::from_ascii(&name.as_str())
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let chunk = chunks
        .into_iter()
        .find(|c| c.chunk_type == chunk_type)
        .ok_or_else(|| PyRuntimeError::new_err(format!("Chunk {} not found", name)))?;
    Ok(PyBytes::new(py, &chunk.data))
}

#[pymodule]
#[pyo3(name = "png_achunk")]
fn module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_chunk_from_png_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(read_chunk, m)?)?;
    Ok(())
}
