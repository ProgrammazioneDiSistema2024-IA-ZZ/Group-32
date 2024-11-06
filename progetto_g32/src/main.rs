mod cpu_logger;
use std::time::Instant;

fn main() {
    // Esegue il logging dell'uso della CPU in un thread separato
    std::thread::spawn(|| {
        cpu_logger::log_cpu_usage();
    });

    // Simula l'inizio del backup
    let start_time = Instant::now();
    let total_size = 2048; // Dimensione del backup in byte (esempio)

    // Codice per il backup (implementato dal tuo collega)
    // ...

    // Quando il backup è completato
    cpu_logger::log_backup_stats(start_time, total_size);
}
