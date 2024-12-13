use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::{mouse_input};

// Variabili globali per i percorsi
lazy_static! {
    pub static ref SOURCE_PATH: String = read_path(0);
    pub static ref DESTINATION_PATH: String = read_path(1);
    pub static ref CHILD_PROCESS_ID: Mutex<u32> = Mutex::new(0);
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

pub fn main_configuration() {
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
            let exe = env::current_exe().unwrap(); // exe path
            let wd = exe.parent().unwrap();
            let program_path = wd.join("setup");

            let child = Command::new(program_path)
                .spawn()
                .expect("Errore durante l'avvio del programma di configurazione");

            println!("Pid del child: {}", child.id());
            {
                let mut child_id_lock = CHILD_PROCESS_ID.lock().unwrap();
                *child_id_lock = child.id();
            }

        }
    } else {
        println!("Impossibile aprire il file.");
    }
}
