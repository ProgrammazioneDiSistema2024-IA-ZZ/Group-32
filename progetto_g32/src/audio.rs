use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn play_sound() {
    // Crea un output audio per riprodurre il suono
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Specifica il percorso del file audio
    let file = BufReader::new(File::open("audio_backup/audio.wav").unwrap());

    // Decodifica e aggiunge il file audio al sink
    let source = rodio::Decoder::new(file).unwrap();
    sink.append(source);

    // Attende la fine della riproduzione
    sink.sleep_until_end();
}