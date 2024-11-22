use std::thread;
use crate::cpu_logger::log_cpu_usage;

mod backup;
mod mouse_input;
mod audio;
mod cpu_logger;
mod confirmation_window;
mod configuration_window;
mod main_configuration;

fn main() {

    let log_thread = thread::spawn(|| {
        log_cpu_usage();
    });

    println!("avvio programma!");
    main_configuration::main();

    // Attende la terminazione del thread di logging (se necessario)
    log_thread.join().expect("Errore nel thread di logging");
}
