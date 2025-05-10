#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use arrayref::array_ref;

#[derive(Debug)]
#[repr(C, packed)]
pub struct ChunkHeader {
    pub chunk: [u8; 4], // "RIFF"
    pub size: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Riff {
    pub id: [u8; 4], // "WAVE"
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Fmt {
    pub compression_code: u16,
    pub number_of_channels: u16,
    pub sampling_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

#[derive(Debug)]
pub enum Chunk<'a> {
    Riff(Riff),
    Format(Fmt),
    List(Vec<(String, String)>),
    Data(&'a [u8]),
}

#[derive(Debug)]
pub struct ChunkInfo {
    pub name: String,
    pub data_offset: usize,
    pub data_length: usize,
}

pub struct WAV<'a> {
    data: &'a [u8],
}

impl<'data_lt> WAV<'_> {
    pub fn from_data(data: &'data_lt [u8]) -> WAV<'data_lt> {
        WAV { data }
    }

    fn read_chunk_hdr(&self, position: usize) -> ChunkHeader {
        let orig_data = &self.data[position..position + size_of::<ChunkHeader>()];

        unsafe { (orig_data.as_ptr() as *const ChunkHeader).read_unaligned() }
    }

    pub fn available_chunks(&self) -> Vec<ChunkInfo> {
        let mut chunks: Vec<_> = vec![];
        let mut index = 0;
        let ch_size = size_of::<ChunkHeader>();

        while index < self.data.len() {
            let chunk = self.read_chunk_hdr(index);
            index += ch_size;

            let data_size: usize;

            if chunk.chunk == *b"RIFF" {
                // RIFF spans whole file
                data_size = 4;
            } else {
                data_size = chunk.size as usize;
            }

            let name = String::from_utf8_lossy(&chunk.chunk);
            chunks.push(ChunkInfo {
                name: name.into_owned(),
                data_offset: index,
                data_length: data_size,
            });

            index += data_size;
        }

        chunks
    }

	fn parse_list(&self, data: &[u8]) -> Option<Chunk<'data_lt>> {
        let mut entities_list = Vec::new();
		
		if data[..4] != *b"INFO" {
			return None;
		}

		let mut index: usize = 4;

		while index < data.len() {
			// ...
            let name = String::from_utf8_lossy(&data[index..index + 4]).to_string();

            index += 4;

            let name_len = u32::from_le_bytes(*array_ref![data[index..index + 4], 0, 4]) as usize;

            index += 4;

            let value = String::from_utf8_lossy(&data[index..index + name_len - 1]).to_string();

            entities_list.push((name, value));

            index += name_len;

            if name_len % 2 == 1 {
                // Padding!
                index += 1;
            }
		}

		return Some(Chunk::List(entities_list));
	}

	pub fn read_chunk(&'data_lt self, chunk: &ChunkInfo) -> Option<Chunk<'data_lt>> {
		let data: &[u8] = &self.data[chunk.data_offset..chunk.data_offset + chunk.data_length];

		match chunk.name.as_str() {
			"RIFF" => {
				Some(Chunk::Riff(
					unsafe { (data.as_ptr() as *const Riff).read_unaligned() }
				))
			},
			"fmt " => {
				Some(Chunk::Format(
					unsafe { (data.as_ptr() as *const Fmt).read_unaligned() }
				))
			},
			"LIST" => self.parse_list(data),
            "data" => Some(Chunk::Data(data)),
			_ => None
		}
	}

    pub fn read_chunk_by_name(&'data_lt self, chunk_name: &str) -> Option<Chunk<'data_lt>> {
        let chunks = self.available_chunks();
        let chunk = chunks.iter().filter(|&a| a.name == chunk_name).next()?;

        self.read_chunk(chunk)
    }
}
