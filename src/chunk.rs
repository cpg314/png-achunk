use std::io::Read;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Non-ASCII chunk type name")]
    TypeAscii,
    #[error("Chunk type name should be of length 4")]
    TypeLength,
    #[error("Data too large")]
    DataLength,
    #[error("Third chunk type letter should be uppercase")]
    ThirdCase,
    #[error("Invalid checksum")]
    Checksum,
    #[error("Failed to decode chunk")]
    Decoding(#[from] std::io::Error),
}

/// Chunk type/name.
///
/// The first letter must be lowercase if the chunk is non-critical.
/// The second letter must be lower case if the chunk specification is not public.
/// The third letter must always be uppercase.
/// The fourth latter must be lowercase if the chunk is safe to copy even when critical chunks have
/// been modified
///
/// For a custom private ancillary chunk, use a name of the form `llU{l/U}`, where l (resp. U)
/// denotes a lowercase (resp. uppercase) letter.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType([ascii::AsciiChar; 4]);

impl ChunkType {
    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
    pub fn to_ascii(&self) -> &ascii::AsciiStr {
        self.0.as_slice().into()
    }
    pub fn from_ascii(name: &impl ascii::AsAsciiStr) -> Result<Self, Error> {
        let name = name.as_ascii_str().map_err(|_| Error::TypeAscii)?;
        if name.len() != 4 {
            return Err(Error::TypeLength);
        }
        if !name[2].is_ascii_uppercase() {
            return Err(Error::ThirdCase);
        }
        Ok(Self(name.as_slice().try_into().unwrap()))
    }
}
impl std::cmp::PartialEq<png::chunk::ChunkType> for ChunkType {
    fn eq(&self, other: &png::chunk::ChunkType) -> bool {
        other == &png::chunk::ChunkType::from(self)
    }
}
impl From<&ChunkType> for png::chunk::ChunkType {
    fn from(source: &ChunkType) -> Self {
        Self(source.0.map(|ch| ch.as_byte()))
    }
}
impl From<ChunkType> for png::chunk::ChunkType {
    fn from(source: ChunkType) -> Self {
        (&source).into()
    }
}

/// PNG chunk
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    pub chunk_type: ChunkType,
    length: u32,
    pub data: Vec<u8>,
}
impl Chunk {
    /// Create a new custom chunk with the attached data, whose length must note exceed u32::MAX.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Result<Self, Error> {
        Ok(Self {
            chunk_type,
            length: data.len().try_into().map_err(|_| Error::DataLength)?,
            data,
        })
    }
    pub fn from_reader(mut reader: impl Read) -> Result<Self, Error> {
        let mut hasher = crc32fast::Hasher::new();

        let mut length = [0; 4];
        reader.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);

        let mut chunk_type = [0; 4];
        reader.read_exact(&mut chunk_type)?;
        hasher.update(&chunk_type);
        let chunk_type = ChunkType::from_ascii(&chunk_type.as_slice())?;

        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data)?;
        hasher.update(&data);

        let mut crc = [0; 4];
        reader.read_exact(&mut crc)?;

        let crc = u32::from_be_bytes(crc);

        if crc != hasher.finalize() {
            return Err(Error::Checksum);
        }

        Ok(Self {
            chunk_type,
            length,
            data,
        })
    }
}
