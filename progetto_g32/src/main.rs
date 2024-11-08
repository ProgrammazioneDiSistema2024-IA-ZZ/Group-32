mod backup;

use std::path::Path;
use std::fs;
use crate::backup::backup;

fn main() {
    // Leggi il file di configurazione
    let source = Path::new("/path/to/source");
    let destination = Path::new("/path/to/usb-drive");
    let file_types = vec!["txt", "jpg", "pdf"];

    // Avvia il backup
    if let Err(e) = backup(source, destination, file_types) {
        eprintln!("Error during backup: {}", e);
    }
}
