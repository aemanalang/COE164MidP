use std::fs::File;

pub struct FlacWriter;

pub struct FlacFileInfo {

    pub flac_header: u32,
    pub flac_meta: FlacMeta,
    pub flac_audio: FlacAudio,

}

pub struct FlacMeta {

    pub flac_meta_temp: u32,

}

pub struct FlacAudio {

    pub flac_audio_temp: u32,

}

impl FlacWriter {

    pub fn create_flac_info(){

        let flac_file_info = FlacFileInfo::new();

    }

    pub fn gen_file(){}

}

impl FlacFileInfo {

    pub fn new() -> Self {

        FlacFileInfo {

            flac_header: 0x664C6143,
            flac_meta: FlacMeta::new(), // make meta blocks struct
            flac_audio: FlacAudio::new(), // make audio blocks struct
        }

    }

}

impl FlacMeta {

    pub fn new() -> Self {

        FlacMeta {

            flac_meta_temp: 1,

        }

    }

}

impl FlacAudio {

    pub fn new() -> Self {

        FlacAudio {

            flac_audio_temp: 1,

        }

    }

}