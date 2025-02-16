use sysinfo::{ProcessesToUpdate, System};
use std::fs::OpenOptions;
use std::io::Write;
use std::{env, thread};
use std::time::Duration;
use chrono::Local;
use sysinfo::Pid;

pub fn log_cpu_usage() {
    let mut system = System::new_all();
    let log_file_path = "cpu_usage_log.txt";

    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap().parent().unwrap().parent().unwrap();
    let path = wd.join(log_file_path);
    println!("{:?}", path);

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Errore nell'apertura del file di log");

    // Ottieni il PID del processo corrente
    let pid = std::process::id(); // Ottieni il PID del processo corrente

    loop {
        system.refresh_processes(
            ProcessesToUpdate::Some(&[Pid::from(pid as usize)]),
            true,
        );
        // Converte il PID in un tipo Pid
        let pid = Pid::from(pid as usize);

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
