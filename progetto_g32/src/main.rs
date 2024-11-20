use std::thread;
use crate::cpu_logger::log_cpu_usage;

mod backup;
mod mouse_input;
mod audio;
mod cpu_logger;

fn main() {

    let log_thread = thread::spawn(|| {
        log_cpu_usage();
    });

    println!("avvio programma!");
    mouse_input::main();

    // Attende la terminazione del thread di logging (se necessario)
    log_thread.join().expect("Errore nel thread di logging");
}
