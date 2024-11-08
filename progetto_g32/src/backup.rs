use std::fs::{self, DirEntry};
use std::path::Path;
use std::time::{Instant, Duration};
use std::io::{self, Write};
use std::fs::File;

pub fn backup(source: &Path, destination: &Path, file_types: Vec<&str>) -> io::Result<()> {
    let start_time = Instant::now();
    let mut total_size = 0;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if file_types.contains(&extension.to_str().unwrap()) {
                    // Copia il file
                    let dest_path = destination.join(path.file_name().unwrap());
                    fs::copy(&path, dest_path)?;

                    total_size += path.metadata()?.len();
                }
            }
        } else if path.is_dir() {
            // Backup delle cartelle (ricorsivo)
            let dir_name = path.file_name().unwrap();
            let new_dest = destination.join(dir_name);
            fs::create_dir_all(&new_dest)?;
            backup(&path, &new_dest, file_types.clone())?; // Ricorsiva per le sottocartelle
        }
    }

    let duration = start_time.elapsed();
    let log_file = File::create(destination.join("backup_log.txt"))?;
    writeln!(log_file, "Backup completed in {:?} with {} bytes copied.", duration, total_size)?;
    writeln!(log_file, "Total time spent on CPU: {:?}", duration)?;

    Ok(())
}
