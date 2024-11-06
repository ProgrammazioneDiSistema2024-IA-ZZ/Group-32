use sysinfo::{System, SystemExt, CpuExt};
use std::{thread, time::Duration};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

// Funzione per loggare il consumo di CPU periodicamente
pub fn log_cpu_usage() {
    let mut system = System::new_all();
    let log_file_path = "cpu_usage_log.txt";
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Errore nell'apertura del file di log");

    loop {
        system.refresh_cpu();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let avg_cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;
        let log_entry = format!("{}, CPU Usage: {:.2}%\n", timestamp, avg_cpu_usage);

        if let Err(e) = log_file.write_all(log_entry.as_bytes()) {
            eprintln!("Errore durante la scrittura nel file di log: {}", e);
        }

        println!("{} - CPU Usage: {:.2}%", timestamp, avg_cpu_usage);
        thread::sleep(Duration::from_secs(120));
    }
}

// Funzione per loggare le statistiche del backup
pub fn log_backup_stats(start_time: std::time::Instant, total_size: u64) {
    let log_file_path = "backup_log.txt";
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Errore nell'apertura del file di log del backup");

    let duration = start_time.elapsed();
    let duration_secs = duration.as_secs();

    let log_entry = format!(
        "Backup completato il {}\nDimensione totale dei file: {} bytes\nTempo di CPU usato: {} secondi\n\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        total_size,
        duration_secs
    );

    if let Err(e) = log_file.write_all(log_entry.as_bytes()) {
        eprintln!("Errore durante la scrittura nel file di log del backup: {}", e);
    }

    println!("Log backup completato: {}", log_entry);
}
