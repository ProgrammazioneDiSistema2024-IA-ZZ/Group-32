use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

pub fn backup(source: &Path, destination: &Path, file_types: Vec<&str>) -> io::Result<()> {
    let start_time = Instant::now();
    let mut total_size = 0;
    let mut log_messages = Vec::new();

    fn recursive_backup(
        source: &Path,
        destination: &Path,
        file_types: &[&str],
        total_size: &mut u64,
        log_messages: &mut Vec<String>,
    ) -> io::Result<()> {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if file_types.contains(&extension.to_str().unwrap()) {
                        let dest_path = destination.join(path.file_name().unwrap());
                        fs::copy(&path, &dest_path)?;

                        let file_size = path.metadata()?.len();
                        *total_size += file_size;
                        log_messages.push(format!(
                            "Copied file: {:?} ({} bytes)",
                            path,
                            file_size
                        ));
                    }
                }
            } else if path.is_dir() {
                let dir_name = path.file_name().unwrap();
                let new_dest = destination.join(dir_name);
                fs::create_dir_all(&new_dest)?;
                recursive_backup(&path, &new_dest, file_types, total_size, log_messages)?;
            }
        }
        Ok(())
    }

    // Creazione della directory principale di destinazione
    fs::create_dir_all(destination)?;

    // Avvio del backup ricorsivo
    recursive_backup(source, destination, &file_types, &mut total_size, &mut log_messages)?;

    // Scrittura del log finale
    let duration = start_time.elapsed();
    let mut log_file = File::create(destination.join("backup_log.txt"))?;
    writeln!(log_file, "Backup completed in {:?} with {} bytes copied.", duration, total_size)?;
    writeln!(log_file, "Detailed log:")?;
    for message in log_messages {
        writeln!(log_file, "{}", message)?;
    }

    Ok(())
}
