use service_manager::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let label: ServiceLabel = "com.example.my-service".parse()?;

    // Rileva e utilizza il gestore di servizi nativo della piattaforma
    let manager = <dyn ServiceManager>::native()
        .expect("Failed to detect management platform");

    // Percorso dell'eseguibile (modifica il percorso per il tuo sistema)
    let _executable_path = "/Users/matteopetrera/Desktop/POLITO/MAGISTRALE/23-24-2semestre/PDS/RUST/Group-32/target/release/progetto_g32";

    // Ferma il servizio
    manager.stop(ServiceStopCtx {
        label: label.clone(),
    }).expect("Errore durante l'arresto del servizio");

    println!("Servizio disinstallato correttamente.");

    Ok(())

}