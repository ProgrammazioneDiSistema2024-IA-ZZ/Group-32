
use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crea un'etichetta per il servizio
    let label: ServiceLabel = "com.progetto_g32.my-service".parse()?;

    // Rileva e utilizza il gestore di servizi nativo della piattaforma
    let manager = <dyn ServiceManager>::native()
        .expect("Failed to detect management platform");

    // Percorso dell'eseguibile (modifica il percorso per il tuo sistema)
    let executable_path = "/Users/matteopetrera/Desktop/POLITO/MAGISTRALE/23-24-2semestre/PDS/RUST/Group-32/target/release/progetto_g32";

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
