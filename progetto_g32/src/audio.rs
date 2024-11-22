use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn play_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Usa la directory del progetto per costruire il percorso relativo
    let mut audio_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    audio_path.push("audio_backup/audio.wav");

    let file = BufReader::new(File::open(audio_path).unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}

pub fn play_sound2() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Usa la directory del progetto per costruire il percorso relativo
    let mut audio_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    audio_path.push("audio_backup/audio2.wav");

    let file = BufReader::new(File::open(audio_path).unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
