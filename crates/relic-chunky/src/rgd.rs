use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    io::{self, BufRead, Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

use crate::chunky::{ChunkFile, ChunkHeader, ChunkType};

#[derive(Debug)]
pub struct RelicGameData {}

#[derive(Error, Debug)]
pub enum RelicGameDataError {
    // #[error("data store disconnected")]
    // Disconnect(),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("No DATA KEYS chunk present")]
    MissingDataKeysChunk,

    #[error("More than one DATA KEYS chunk present")]
    MultipleDataKeysChunks,

    #[error("No DATA AEGD chunk present")]
    MissingDataAegdChunk,

    #[error("More than one DATA AEGD chunk present")]
    MultipleDataAegdChunks,

    #[error("Invalid data type")]
    InvalidDataType(#[from] Box<dyn std::error::Error>),

    #[error("Unknown data type {0}")]
    UnknownDataType(i32),
}

pub enum RGDDataType {
    Float(f32),
    Int(i32),
    Boolean(bool),
    CString(String),
    List(Vec<RGDDataType>),
    List2(Vec<RGDDataType>),
}

impl RelicGameData {
    pub fn parse<R: Read + BufRead + Seek>(
        chunk_file: &mut ChunkFile<R>,
    ) -> Result<Self, RelicGameDataError> {
        let mut keys_chunk_header = None;
        let mut kvs_chunk_header = None;

        for chunk in &chunk_file.chunks {
            if chunk.chunk_type == ChunkType::Data {
                if chunk.name == "KEYS" {
                    if let Some(_) = keys_chunk_header {
                        return Err(RelicGameDataError::MultipleDataKeysChunks);
                    }
                    keys_chunk_header = Some(chunk);
                } else if chunk.name == "AEGD" {
                    if let Some(_) = kvs_chunk_header {
                        return Err(RelicGameDataError::MultipleDataAegdChunks);
                    }
                    kvs_chunk_header = Some(chunk);
                }
            }
        }

        let keys_chunk_header = match keys_chunk_header {
            Some(keys) => keys,
            None => return Err(RelicGameDataError::MissingDataKeysChunk),
        };

        let kvs_chunk_header = match kvs_chunk_header {
            Some(kvs) => kvs,
            None => return Err(RelicGameDataError::MissingDataAegdChunk),
        };

        let keys = Self::parse_keys(&mut chunk_file.reader, keys_chunk_header);
        println!("keys: {:?}", keys);

        let kvs = Self::parse_aegd(&mut chunk_file.reader, kvs_chunk_header);
        println!("kvs: {:?}", kvs);

        // println!("kvs: {:?}", kvs);
        Ok(Self {})
    }

    pub fn read_chunky_list<R: BufRead + Read + Seek>(
        reader: &mut R,
        chunk: &ChunkHeader,
    ) -> Result<HashMap<u64, String>, RelicGameDataError> {
        let length = reader.read_u32::<LittleEndian>()?;

        let mut key_type_and_data_index = Vec::<(u64, RGDDataType, i32)>::new();

        for _ in 0..length {
            let key = reader.read_u64::<LittleEndian>()?;
            let data_type = reader.read_i32::<LittleEndian>()?;
            let data_index = reader.read_i32::<LittleEndian>()?;

            match data_type {
                0 => {
                    let value = reader.read_f32::<LittleEndian>()?;
                    key_type_and_data_index.push((key, RGDDataType::Float(value), data_index));
                }
                1 => {
                    let value = reader.read_i32::<LittleEndian>()?;
                    key_type_and_data_index.push((key, RGDDataType::Int(value), data_index));
                }
                2 => {
                    let value = reader.read_u8()?;
                    key_type_and_data_index.push((
                        key,
                        RGDDataType::Boolean(value != 0),
                        data_index,
                    ));
                }
                3 => {
                    let mut string_bytes = Vec::new();
                    reader.read_until(b'\0', &mut string_bytes)?;
                    let value = CString::new(string_bytes)
                        .map_err(|e| RelicGameDataError::InvalidDataType(Box::new(e)))?
                        .into_string()
                        .map_err(|e| RelicGameDataError::InvalidDataType(Box::new(e)))?;

                    key_type_and_data_index.push((key, RGDDataType::CString(value), data_index));
                }
                100 => {
                    let value = Vec::<RGDDataType>::new();
                    key_type_and_data_index.push((key, RGDDataType::List(value), data_index));
                }
                101 => {
                    let value = Vec::<RGDDataType>::new();
                    key_type_and_data_index.push((key, RGDDataType::List2(value), data_index));
                }
                _ => {
                    return Err(RelicGameDataError::UnknownDataType(data_type));
                }
            };
        }

        Ok(())
    }

    pub fn parse_aegd<R: BufRead + Read + Seek>(
        reader: &mut R,
        chunk: &ChunkHeader,
    ) -> Result<HashMap<u64, String>, RelicGameDataError> {
        let key_string_map = HashMap::new();
        reader.seek(SeekFrom::Start(chunk.data_position_start))?;

        let unknown = reader.read_u32::<LittleEndian>()?;

        println!("unknown: {:?}", unknown);

        Ok(key_string_map)
    }

    pub fn parse_keys<R: Read + Seek>(
        reader: &mut R,
        chunk: &ChunkHeader,
    ) -> Result<HashMap<u64, String>, RelicGameDataError> {
        let mut key_string_map = HashMap::new();
        reader.seek(SeekFrom::Start(chunk.data_position_start))?;

        let count = reader.read_u32::<LittleEndian>()?;

        for _ in 0..count {
            let key = reader.read_u64::<LittleEndian>()?;
            let string_length = reader.read_u32::<LittleEndian>()?;

            let string = {
                let mut string_bytes = vec![0u8; string_length as usize];
                reader.read_exact(&mut string_bytes)?;
                String::from_utf8_lossy(&string_bytes).to_string()
            };

            key_string_map.insert(key, string);
        }

        Ok(key_string_map)
    }
}
