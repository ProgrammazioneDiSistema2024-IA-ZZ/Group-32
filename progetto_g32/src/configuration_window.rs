use crate::{main, mouse_input};
use eframe::egui;
use rfd::FileDialog;
use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::PathBuf;
use std::process;
use eframe::egui::{Align, Layout, ViewportCommand};
use crate::main_configuration::CHILD_PROCESS_ID;

pub fn run_configuration_window() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Configurazione Backup",
        options,
        Box::new(|_cc| Box::new(ConfigurationApp::default())),
    ).expect("TODO: panic message");
}

struct ConfigurationApp {
    source_path: Option<String>,
    destination_path: Option<String>,
}

impl Default for ConfigurationApp {
    fn default() -> Self {
        Self {
            source_path: None,
            destination_path: None,
        }
    }
}

impl eframe::App for ConfigurationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Configurazione Backup");

            // Percorso sorgente
            ui.horizontal(|ui| {
                ui.label("Percorso sorgente:");
                if ui.button("Scegli...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.source_path = Some(path.display().to_string());
                        println!("Percorso sorgente selezionato: {:?}", self.source_path);
                    }
                }
            });
            if let Some(ref path) = self.source_path {
                ui.label(format!("Sorgente: {}", path));
            }

            // Percorso destinazione
            ui.horizontal(|ui| {
                ui.label("Percorso destinazione:");
                if ui.button("Scegli...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.destination_path = Some(path.display().to_string());
                        println!("Percorso destinazione selezionato: {:?}", self.destination_path);
                    }
                }
            });
            if let Some(ref path) = self.destination_path {
                ui.label(format!("Destinazione: {}", path));
            }

            ui.separator();

            // Pulsante Installa
            if ui.button("Salva").clicked() {
                if let (Some(source), Some(destination)) = (&self.source_path, &self.destination_path) {
                    println!("Setup completato con i seguenti percorsi:");
                    println!("Sorgente: {}", source);
                    println!("Destinazione: {}", destination);

                    // Salva i percorsi in un file CSV
                    if let Err(e) = save_to_csv(source, destination) {
                        eprintln!("Errore nel salvataggio del file CSV: {}", e);
                    } else {
                        println!("Percorsi salvati con successo in backup_config.csv");
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                        //mouse_input::main();
                    }
                } else {
                    println!("Errore: seleziona entrambi i percorsi prima di continuare.");
                }
            }
        });
    }
}

/// Salva i percorsi in un file CSV
fn save_to_csv(source: &str, destination: &str) -> std::io::Result<()> {

    // Usa la directory del progetto per costruire il percorso relativo
    let mut csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    csv_path.push("configuration_csv/configuration.csv");

    // Apri il file in modalità append, creando se necessario
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&csv_path)?;
    // Se ci sono linee già scritte sul file eliminale
    // Scrivi i dati sul file
    writeln!(file, "{},{}", source, destination)?;

    println!("Percorsi salvati in: {:?}", csv_path);
    Ok(())

}
