use std::path::PathBuf;

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes, types::PyString};

#[pyfunction]
fn read_chunk<'a>(
    py: Python<'a>,
    filename: &'a PyString,
    name: &'a PyString,
) -> PyResult<&'a PyBytes> {
    let chunks = png_achunk::Decoder::from_file(filename.extract::<PathBuf>()?)?
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
    m.add_function(wrap_pyfunction!(read_chunk, m)?)?;
    Ok(())
}
