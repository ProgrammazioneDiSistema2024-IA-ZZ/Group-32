use std::path::Path;
use std::thread;

mod backup;
use backup::{execute_backup_with_logging, log_cpu_usage};

fn main() {
    // Configura i percorsi per la sorgente e destinazione del backup
    let src = Path::new("/path/to/source");      // Cambia con il percorso sorgente reale
    let dest = Path::new("/media/usb");          // Cambia con il percorso di destinazione su USB
    let cpu_log_path = Path::new("/path/to/cpu_log.txt");

    // Avvia il log del consumo CPU in un thread separato
    let _cpu_log_handle = thread::spawn(move || log_cpu_usage(cpu_log_path));

    // Esegui il backup con logging del tempo di CPU e dimensione totale
    if let Err(e) = execute_backup_with_logging(src, dest, cpu_log_path) {
        eprintln!("Errore durante il backup: {:?}", e);
    }
}
