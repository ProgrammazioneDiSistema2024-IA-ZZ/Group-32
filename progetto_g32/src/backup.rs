use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use sysinfo::{ProcessorExt, System, SystemExt};
use std::thread;

pub fn backup_directory(src: &Path, dest: &Path) -> io::Result<u64> {
    let mut total_size = 0;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)?;
            total_size += backup_directory(&file_path, &dest_path)?;
        } else if file_type.is_file() {
            fs::copy(&file_path, &dest_path)?;
            total_size += fs::metadata(&dest_path)?.len();
        }
    }

    Ok(total_size)
}

pub fn log_cpu_usage(log_path: &Path) {
    let mut system = System::new_all();
    let mut log_file = File::create(log_path).expect("Impossibile creare il file di log del consumo CPU");

    loop {
        system.refresh_all();
        let cpu_usage = system.global_processor_info().cpu_usage();
        let timestamp = chrono::Utc::now().to_rfc3339();

        writeln!(log_file, "{}: CPU Usage: {:.2}%", timestamp, cpu_usage).expect("Errore nella scrittura del log CPU");
        log_file.flush().expect("Errore nel flush del file di log CPU");

        thread::sleep(Duration::from_secs(120)); // Logga ogni 2 minuti
    }
}

pub fn execute_backup_with_logging(src: &Path, dest: &Path, log_path: &Path) -> io::Result<()> {
    // Controllo preliminare dei permessi di scrittura
    if let Err(e) = check_write_permission(dest) {
        eprintln!("Errore: File system di sola lettura o permessi insufficienti. Backup non possibile.");
        return Err(e);
    }

    let start_time = Instant::now();
    let total_size = backup_directory(src, dest)?;
    let elapsed_time = start_time.elapsed().as_secs();

    // Scrive il log del backup
    let mut log_file = File::create(log_path)?;
    writeln!(
        log_file,
        "Backup completato: {} bytes copiati in {} secondi",
        total_size, elapsed_time
    )?;

    Ok(())
}

pub fn check_write_permission(dest: &Path) -> io::Result<()> {
    let test_path = dest.join("test_permission.txt");
    let result = File::create(&test_path).and_then(|mut file| file.write_all(b"test"));
    fs::remove_file(test_path).ok(); // Rimuove il file di test se creato
    result
}



