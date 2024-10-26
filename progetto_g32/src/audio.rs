use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn play_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("audio_backup/audio.wav").unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
