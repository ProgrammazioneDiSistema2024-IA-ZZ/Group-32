use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;
use lazy_static::lazy_static;
use crate::configuration_window::run_configuration_window;
use crate::{mouse_input};

// Variabili globali per i percorsi
lazy_static! {
    pub static ref SOURCE_PATH: String = read_path(0);
    pub static ref DESTINATION_PATH: String = read_path(1);
}

fn read_path(index: usize) -> String {
    // Usa la directory del progetto per costruire il percorso relativo
    let mut csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    csv_path.push("configuration_csv/configuration.csv");

    if let Ok(file) = File::open(csv_path) {
        let reader = BufReader::new(file);
        if let Some(Ok(line)) = reader.lines().next() {
            let paths: Vec<&str> = line.split(',').collect();
            if index < paths.len() {
                return paths[index].trim().to_string();
            }
        }
    }

    panic!("Errore: impossibile leggere il percorso al numero di indice {}", index);
}

pub fn main() {
    // Usa la directory del progetto per costruire il percorso relativo
    let mut csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    csv_path.push("configuration_csv/configuration.csv");

    // Controlla se ci sono righe di testo nel file
    if let Ok(file) = File::open(csv_path) {
        let reader = BufReader::new(file);
        let has_lines = reader.lines().next().is_some();

        if has_lines {
            mouse_input::main();
        } else {
            println!("Il file è vuoto o non contiene righe di testo.");
            run_configuration_window();
        }
    } else {
        println!("Impossibile aprire il file.");
    }
}
