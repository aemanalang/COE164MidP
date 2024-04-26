use core::fmt;
use std::fs::File;
use std::path::Path;
use std::error;
use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{ByteOrder, LittleEndian};

pub struct PCMWaveInfo {
    pub riff_header: RiffChunk,
    pub fmt_header: PCMWaveFormatChunk,
    pub data_chunks: Vec <PCMWaveDataChunk>,
}

pub struct RiffChunk {
    pub file_size: u32,
    pub is_big_endian: bool,
}

#[derive(Clone, Copy)]
pub struct PCMWaveFormatChunk {
    pub num_channels: u16,
    pub samp_rate: u32,
    pub bps: u16,
}

pub struct PCMWaveDataChunk {
    pub size_bytes: u32,
    pub format: PCMWaveFormatChunk,
    pub data_buf: io::BufReader<File>,
}

pub struct PCMWaveDataChunkSamples {
    data_chunk: PCMWaveDataChunk,
}

pub struct WaveReader;

#[derive(Debug)]
pub enum WaveReaderError {
    NotRiffError,
    NotWaveError,
    NotPCMError,
    ChunkTypeError,
    DataAlignmentError,
    ReadError,
}

impl WaveReader {
    pub fn open_pcm(file_path: &str) -> Result <PCMWaveInfo, WaveReaderError> {
        todo!();
    }

    fn read_riff_chunk(fh: &mut File) -> Result <RiffChunk, WaveReaderError> {
        todo!();
    }

    fn read_fmt_chunk(fh: &mut File) -> Result <PCMWaveFormatChunk, WaveReaderError> {
        todo!();
    }

    fn read_data_chunk(start_pos: u64, fmt_info: &PCMWaveFormatChunk, mut fh: File) -> Result <PCMWaveDataChunk, WaveReaderError> {
        todo!();
    }
}

impl PCMWaveFormatChunk {
    fn byte_rate(&self) -> u32 {
        todo!();
    }

    fn block_align(&self) -> u16 {
        todo!();
    }
}

impl PCMWaveDataChunk {
    pub fn byte_rate(self) -> PCMWaveDataChunkSamples {
        todo!();
    }
}

mod tests {
    // TODO: Add tests here
}