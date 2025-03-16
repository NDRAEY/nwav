#![no_std]

pub struct Chunk {
	pub chunk: [u8; 4],   // "RIFF"
	pub size: u32,	
}

pub struct Riff {
	pub id: [u8; 4]   // "wave"
}

pub struct Fmt {
	pub compression_code: u16,
	pub number_of_channels: u16,
	pub sampling_rate: u32,
	pub byte_rate: u32,
	pub block_align: u16,
	pub bits_per_sample: u16
}
