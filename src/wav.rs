use core::fmt;
use std::fs::File;
use std::path::Path;
use std::error;
use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{ByteOrder, LittleEndian};

/// Represents a PCM WAV file
pub struct PCMWaveInfo {
    pub riff_header: RiffChunk,
    pub fmt_header: PCMWaveFormatChunk,
    pub data_chunks: Vec <PCMWaveDataChunk>,
}

/// Represents a RIFF chnk from a WAV file
/// 
/// The RIFF chunk is the first 12 bytes of a WAV file.
pub struct RiffChunk {
    pub file_size: u32,
    pub is_big_endian: bool,
}

/// Represents a format chunk from a WAV file
/// 
/// A format chunk in a WAV file starts with a magic string
/// `fmt_` where `_` is a space (0x20 in hex) and then followed by
/// 20 bytes of metadata denoting information about the audio file
/// itself such as the sample and bit rates.
#[derive(Clone, Copy)]
pub struct PCMWaveFormatChunk {
    pub num_channels: u16,
    pub samp_rate: u32,
    pub bps: u16,
}

/// Represents a data chunk from a WAV file
/// 
/// A data chunk in a WAV file starts with a magic string `data` and then
/// followed by the number of samples that follow and then finally the
/// audio data samples themselves.
pub struct PCMWaveDataChunk {
    pub size_bytes: u32,
    pub format: PCMWaveFormatChunk,
    pub data_buf: io::BufReader<File>,
}

/// Represents an iterator to a data chunk from a WAV file
/// 
/// This struct is not instantiated by itself and is generated
/// by calling the methods `PCMWaveDataChunk::chunks_byte_rate()`
/// and `PCMWaveDataChunk::chunks()`.
pub struct PCMWaveDataChunkWindow {
    chunk_size: usize,
    data_chunk: PCMWaveDataChunk, // Borrow a mutable reference
}

/// Represents a WAV reader
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
        let mut fh = File::open(Path::new(file_path))?;
        let riff_header = Self::read_riff_chunk(&mut fh)?;
        let fmt_header = Self::read_fmt_chunk(&mut fh)?;
        let data_chunks = vec![Self::read_data_chunk(36, &fmt_header, fh)?]; // Adjust the starting position as necessary

        Ok(PCMWaveInfo {
            riff_header,
            fmt_header,
            data_chunks,
        })
    }

    fn read_riff_chunk(fh: &mut File) -> Result <RiffChunk, WaveReaderError> {
        let mut buffer = [0u8; 12];
        fh.read_exact(&mut buffer)?;

        if &buffer[0..4] != b"RIFF" && &buffer[0..4] != b"RIFX" {
            return Err(WaveReaderError::NotRiffError);
        }

        let is_big_endian = &buffer[0..4] == b"RIFX"; 

        let file_size = if is_big_endian {
            u32::from_be_bytes(buffer[4..8].try_into().unwrap())
        } else {
            LittleEndian::read_u32(&buffer[4..8])
        };

        if &buffer[8..12] != b"WAVE" { // "WAVE"
        return Err(WaveReaderError::NotWaveError);
    }

        Ok(RiffChunk {
            file_size,
            is_big_endian: is_big_endian, 
        })
    }

    fn read_fmt_chunk(fh: &mut File) -> Result <PCMWaveFormatChunk, WaveReaderError> {
        let mut buffer = [0u8; 24];
        fh.read_exact(&mut buffer)?;

        let chunk_id = LittleEndian::read_u32(&buffer[0..4]);
        if chunk_id != 0x20746D66 { // "fmt "
            return Err(WaveReaderError::ChunkTypeError);
        }

        let audio_format = LittleEndian::read_u16(&buffer[8..10]);
        if audio_format != 1 { // PCM
            return Err(WaveReaderError::NotPCMError);
        }

        let num_channels = LittleEndian::read_u16(&buffer[10..12]);
        let samp_rate = LittleEndian::read_u32(&buffer[12..16]);
        let bps = LittleEndian::read_u16(&buffer[22..24]);

        Ok(PCMWaveFormatChunk {
            num_channels,
            samp_rate,
            bps,
        })
    }

    fn read_data_chunk(start_pos: u64, fmt_info: &PCMWaveFormatChunk, mut fh: File) -> Result<PCMWaveDataChunk, WaveReaderError> {
        let mut buf_reader = io::BufReader::new(fh);
        buf_reader.seek(SeekFrom::Start(start_pos))?;
    
        let mut buffer = [0u8; 8];
        buf_reader.read_exact(&mut buffer)?;
        let chunk_id = LittleEndian::read_u32(&buffer[0..4]);
        let size_bytes = LittleEndian::read_u32(&buffer[4..8]);
    
        if chunk_id != 0x61746164 { // "data"
            return Err(WaveReaderError::ChunkTypeError);
        }
    
        Ok(PCMWaveDataChunk {
            size_bytes,
            format: *fmt_info,
            data_buf: buf_reader,
        })
    }
    
}


impl error::Error for WaveReaderError {}

impl fmt::Display for WaveReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WaveReaderError::NotRiffError => write!(f, "Not a RIFF format error"),
            WaveReaderError::NotWaveError => write!(f, "Not a Wave file error"),
            WaveReaderError::NotPCMError => write!(f, "Not a PCM data error"),
            WaveReaderError::ChunkTypeError => write!(f, "Invalid chunk type error"),
            WaveReaderError::DataAlignmentError => write!(f, "Data alignment error"),
            WaveReaderError::ReadError => write!(f, "Error reading from file."),
        }
    }
}

impl From<io::Error> for WaveReaderError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => WaveReaderError::ReadError,
            io::ErrorKind::PermissionDenied => WaveReaderError::ReadError,
            _ => WaveReaderError::ReadError,
        }
    }
}

impl fmt::Display for PCMWaveInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WAVE File {:?} bytes, {:?}-bit {:?} channels, {:?}Hz, {:?} data chunks", self.riff_header.file_size, self.fmt_header.bps, self.fmt_header.num_channels, self.fmt_header.samp_rate, self.data_chunks.len())
    }
}


impl PCMWaveFormatChunk {
    /// Get or calculate the byte rate of this PCM WAV file
    fn byte_rate(&self) -> u32 {
        self.samp_rate * u32::from(self.num_channels) * u32::from(self.bps) / 8
    }

    /// Get or calculate the block alignment of this PCM WAV file
    /// 
    /// The *block alignment* is the size of one *inter-channel* sample
    /// in bytes. An *inter-channel sample* is a sample with all of its
    /// channels collated together.
    fn block_align(&self) -> u16 {
        self.num_channels * self.bps / 8
    }
}

impl Iterator for PCMWaveDataChunk {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes_per_sample = (self.format.bps / 8) as usize;
        let total_channels = self.format.num_channels as usize;
        let mut buffer = vec![0u8; self.format.block_align() as usize];

        self.data_buf.read_exact(&mut buffer).ok().map(|_| {
            buffer.chunks_exact(bytes_per_sample).map(|sample_bytes| {
                match total_channels {
                    1 => sample_bytes[0] as i64,
                    2 => LittleEndian::read_i16(sample_bytes) as i64,
                    _ => sample_bytes[0] as i64,  
                }
            }).collect()
        })
    }
}

impl Iterator for PCMWaveDataChunkWindow {
    type Item = Vec<Vec<i64>>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_samples = self.chunk_size;
        let mut buffer: Vec<Vec<i64>> = Vec::new();

        for _ in 0..num_samples {
            if let Some(samples) = self.data_chunk.next() {
                buffer.push(samples);
            } else {
                return if buffer.is_empty() { None } else { Some(buffer) }; 
            }
        }
        Some(buffer)
    }
}

impl PCMWaveDataChunk {
    pub fn chunks_byte_rate(self) -> PCMWaveDataChunkWindow { 
        
        PCMWaveDataChunkWindow {
            chunk_size: self.format.byte_rate() as usize,
            data_chunk: self, 
        }
    }

    pub fn chunks(self, chunk_size: usize) -> PCMWaveDataChunkWindow { 
        // samp_rate
        PCMWaveDataChunkWindow {
            chunk_size: chunk_size,
            data_chunk: self, 
        }
    }
}
// TODO: Add more tests here!
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod read_riff {
        use super::*;
        use std::io::Write;

        fn create_temp_file(file_name: &str, content: &[u8]) -> Result <(), io::Error> {
            let mut file = File::create(file_name)?;
            file.write_all(content)?;

            Ok(())
        }
        
        macro_rules! internal_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() -> Result <(), WaveReaderError> {
                    let (input, (will_panic, expected)) = $value;

                    let file_name = format!("midp_{}.wav.part", stringify!($name));
                    let result;
                    {
                        create_temp_file(&file_name, input)?;
                        let mut input_fh = File::open(&file_name)?;
                        result = WaveReader::read_riff_chunk(&mut input_fh);
                    }
                    std::fs::remove_file(&file_name)?;

                    if will_panic {
                        assert!(result.is_err());
                    }
                    else if let Ok(safe_result) = result {
                        assert_eq!(expected.file_size, safe_result.file_size);
                        assert_eq!(expected.is_big_endian, safe_result.is_big_endian);
                    }
                    else {
                        result?;
                    }

                    Ok(())
                }
            )*
            }
        }
        
        internal_tests! {
            it_valid_le_00: (
                &[0x52, 0x49, 0x46, 0x46, 0x0, 0x0, 0x0, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: false,
                    },
                )),
            it_valid_le_01: (
                &[0x52, 0x49, 0x46, 0x46, 0x80, 0x0, 0x0, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 128,
                        is_big_endian: false,
                    },
                )),
            it_valid_le_02: (
                &[0x52, 0x49, 0x46, 0x46, 0x1C, 0x40, 0x36, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 3_555_356,
                        is_big_endian: false,
                    },
                )),
            it_valid_be_00: (
                &[0x52, 0x49, 0x46, 0x58, 0x0, 0x0, 0x0, 0x0, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: true,
                    },
                )),
            it_valid_be_01: (
                &[0x52, 0x49, 0x46, 0x58, 0x00, 0x0, 0x0, 0x80, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 128,
                        is_big_endian: true,
                    },
                )),
            it_valid_be_02: (
                &[0x52, 0x49, 0x46, 0x58, 0x00, 0x36, 0x40, 0x1C, 0x57, 0x41, 0x56, 0x45],
                (
                    false,
                    RiffChunk {
                        file_size: 3_555_356,
                        is_big_endian: true,
                    },
                )),
            it_bad_riff: (
                &[0x00, 0x49, 0x46, 0x46, 0x00, 0x36, 0x40, 0x1C, 0x57, 0x41, 0x56, 0x45],
                (
                    true,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: false,
                    },
                )),
            it_bad_wave: (
                &[0x52, 0x49, 0x46, 0x46, 0x00, 0x36, 0x40, 0x1C, 0x57, 0x41, 0x56, 0x00],
                (
                    true,
                    RiffChunk {
                        file_size: 0,
                        is_big_endian: false,
                    },
                )),
        }
    }

    #[cfg(test)]
    mod read_wav_fmt {
        use super::*;
        use std::io::Write;

        fn create_temp_file(file_name: &str, content: &[u8]) -> Result <(), io::Error> {
            let mut file = File::create(file_name)?;
            file.write_all(content)?;

            Ok(())
        }
        
        macro_rules! internal_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() -> Result <(), WaveReaderError> {
                    let (input, (will_panic, expected)) = $value;

                    let file_name = format!("midp_{}.wav.part", stringify!($name));
                    let result;
                    {
                        create_temp_file(&file_name, input)?;
                        let mut input_fh = File::open(&file_name)?;
                        result = WaveReader::read_fmt_chunk(&mut input_fh);
                    }
                    std::fs::remove_file(&file_name)?;

                    if will_panic {
                        assert!(result.is_err());
                    }
                    else if let Ok(safe_result) = result {
                        assert_eq!(expected.num_channels, safe_result.num_channels);
                        assert_eq!(expected.samp_rate, safe_result.samp_rate);
                        assert_eq!(expected.bps, safe_result.bps);
                    }
                    else {
                        result?;
                    }

                    Ok(())
                }
            )*
            }
        }
        
        internal_tests! {
            it_valid_00: (
                &[
                    0x66, 0x6d, 0x74, 0x20,
                    0x10, 0x0, 0x0, 0x0,
                    0x01, 0x0,
                    0x01, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x01, 0x00, 0x08, 0x0,
                ],
                (
                    false,
                    PCMWaveFormatChunk {
                        num_channels: 1,
                        samp_rate: 44100,
                        bps: 8,
                    },
                )),
            it_valid_01: (
                &[
                    0x66, 0x6d, 0x74, 0x20,
                    0x10, 0x0, 0x0, 0x0,
                    0x01, 0x0,
                    0x02, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x88, 0x58, 0x01, 0x0,
                    0x02, 0x00, 0x08, 0x0,
                ],
                (
                    false,
                    PCMWaveFormatChunk {
                        num_channels: 2,
                        samp_rate: 44100,
                        bps: 8,
                    },
                )),
            it_valid_02: (
                &[
                    0x66, 0x6d, 0x74, 0x20,
                    0x10, 0x0, 0x0, 0x0,
                    0x01, 0x0,
                    0x02, 0x0,
                    0x44, 0xac, 0x0, 0x0,
                    0x10, 0xb1, 0x02, 0x0,
                    0x04, 0x00, 0x10, 0x0,
                ],
                (
                    false,
                    PCMWaveFormatChunk {
                        num_channels: 2,
                        samp_rate: 44100,
                        bps: 16,
                    },
                )),
        }
    }

    mod read_data_fmt {
        // TODO
    }
}