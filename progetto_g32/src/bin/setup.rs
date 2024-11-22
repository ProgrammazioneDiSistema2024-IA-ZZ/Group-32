
use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crea un'etichetta per il servizio
    let label: ServiceLabel = "com.progetto_g32.my-service".parse()?;

    // Rileva e utilizza il gestore di servizi nativo della piattaforma
    let manager = <dyn ServiceManager>::native()
        .expect("Failed to detect management platform");

    // Costruisci il percorso relativo per l'eseguibile
    let mut executable_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    executable_path.push("target/release/progetto_g32"); // Aggiungi il percorso relativo

    // Installa il servizio
    manager.install(ServiceInstallCtx {
        label: label.clone(),
        program: PathBuf::from(executable_path),
        args: vec![OsString::from("--some-arg")],
        contents: None,
        username: None,
        working_directory: None,
        environment: None,
        autostart: true, // Impostato su true per far partire il servizio al riavvio
    })?;

    // Avvia il servizio
    manager.start(ServiceStartCtx {
        label: label.clone(),
    })?;

    println!("Servizio installato e avviato correttamente.");

    Ok(())
}
