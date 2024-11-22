use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

use crate::configuration_window::run_configuration_window;
use crate::mouse_input;


pub fn main() {
    // Usa la directory del progetto per costruire il percorso relativo
    let mut csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    csv_path.push("configuration_csv/configuration.csv");

    // Controlla se ci sono righe di testo nel file
    if let Ok(file) = File::open(csv_path){
      let reader = BufReader::new(file);
      let has_lines = reader.lines().next().is_some();
      
      if has_lines {
          println!("Il file contiene righe di testo.");
          mouse_input::main();
      } else {
          println!("Il file è vuoto o non contiene righe di testo.");
          run_configuration_window();
      }
  } else {
      println!("Impossibile aprire il file.");
  }

}