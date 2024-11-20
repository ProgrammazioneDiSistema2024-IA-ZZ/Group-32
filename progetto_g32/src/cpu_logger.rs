use sysinfo::{System, SystemExt, ProcessExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use chrono::Local;
use sysinfo::Pid;

pub fn log_cpu_usage() {
    let mut system = System::new_all();
    let log_file_path = "cpu_usage_log.txt";
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Errore nell'apertura del file di log");

    // Ottieni il PID del processo corrente
    let pid = std::process::id(); // Ottieni il PID del processo corrente

    loop {
        system.refresh_processes(); // Rinfresca la lista dei processi

        // Converte il PID in un tipo Pid
        let pid = Pid::from(pid as i32);

        // Cerca il processo con il PID specificato nella mappa dei processi
        if let Some(process) = system.processes().get(&pid) {
            // Ottieni l'uso della CPU per il processo
            let cpu_usage = process.cpu_usage();
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            let log_entry = format!("{}, CPU Usage (PID {}): {:.2}%\n", timestamp, pid, cpu_usage);

            // Scrivi nel file di log
            if let Err(e) = log_file.write_all(log_entry.as_bytes()) {
                eprintln!("Errore durante la scrittura nel file di log: {}", e);
            }

            println!("{} - CPU Usage (PID {}): {:.2}%", timestamp, pid, cpu_usage);
        } else {
            eprintln!("Processo con PID {} non trovato", pid);
        }

        // Pausa prima di effettuare il prossimo aggiornamento
        thread::sleep(Duration::from_secs(120));
    }
}
