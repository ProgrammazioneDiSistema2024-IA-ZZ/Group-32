use std::path::Path;
use std::sync::mpsc::Receiver;
use crate::backup::backup;

pub fn run(rx: Receiver<()>) {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::emath::Vec2::new(400.0, 300.0)), // Imposta la dimensione iniziale della finestra
        min_window_size: Some(eframe::emath::Vec2::new(300.0, 200.0)), // Imposta una dimensione minima
        max_window_size: Some(eframe::emath::Vec2::new(800.0, 600.0)), // Imposta una dimensione massima
        ..Default::default()
    };

    eframe::run_native(
        "Conferma Backup",
        options,
        Box::new(move |_cc| Box::new(ConfirmationApp { rx })),
    );
}

struct ConfirmationApp {
    rx: Receiver<()>,
}

impl eframe::App for ConfirmationApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Mostra sempre la finestra di conferma
        eframe::egui::Window::new("Conferma Backup")
            .collapsible(false)
            .default_width(400.0) // Imposta la larghezza della finestra
            .default_height(300.0) // Imposta l'altezza della finestra
            .show(ctx, |ui| {
                ui.label("Sei sicuro di voler confermare il backup?");
                if ui.button("Conferma").clicked() {
                    println!("Backup confermato.");
                    // Definisco i percorsi di origine e destinazione
                    let source = Path::new("/Users/matteopetrera/Desktop/POLITO/MAGISTRALE/23-24-2semestre/PDS/RUST/test-backup-dir");
                    let destination = Path::new("/Users/matteopetrera/Desktop");
                    let file_types = vec!["txt", "jpg", "png"]; // Specifichi i tipi di file

                    // Chiama la funzione di backup
                    match backup(source, destination, file_types) {
                        Ok(_) => println!("Backup eseguito con successo!"),
                        Err(e) => eprintln!("Errore durante il backup: {:?}", e),
                    }
                    std::process::exit(0);
                }
                if ui.button("Annulla").clicked() {
                    println!("Backup annullato.");
                    std::process::exit(0);
                }
            });

        // Controlla se ci sono messaggi dal ricevitore
        if let Ok(_) = self.rx.try_recv() {
            println!("Finestra di conferma ricevuta.");
        }
    }
}
