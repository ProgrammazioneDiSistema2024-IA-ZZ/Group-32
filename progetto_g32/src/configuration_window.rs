
use rfd::FileDialog;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use eframe::egui::{self, ViewportCommand};

pub fn run_configuration_window() {
  let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default().with_inner_size([350f32, 300f32]),
      ..Default::default()
  };

  eframe::run_native(
      "Configurazione Backup",
      options,
      Box::new(|_cc| Ok(Box::new(ConfigurationApp::default()))),
  ).expect("Errore durante l'esecuzione della finestra di configurazione");
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
          // Migliora le spaziature e lo stile globale
          let style = ui.style_mut();
          style.spacing.item_spacing = egui::vec2(8.0, 12.0); // Spaziatura tra gli elementi
          style.visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(240); // Sfondo chiaro

          // Titolo della finestra
          ui.vertical_centered(|ui| {
              ui.heading("Configurazione Backup");
              ui.separator();
          });

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
              ui.colored_label(egui::Color32::LIGHT_GREEN, format!("Sorgente: {}", path));
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
              ui.colored_label(egui::Color32::LIGHT_RED, format!("Destinazione: {}", path));
          }

          ui.separator();

          // Pulsante Salva
          if ui.add_sized([100.0, 30.0], egui::Button::new("Salva")).clicked() {
              if let (Some(source), Some(destination)) = (&self.source_path, &self.destination_path) {
                  println!("Setup completato con i seguenti percorsi:");
                  println!("Sorgente: {}", source);
                  println!("Destinazione: {}", destination);

                  // Salva i percorsi in un file CSV
                  if let Err(e) = save_to_csv(source, destination) {
                      eprintln!("Errore nel salvataggio del file CSV: {}", e);
                  } else {
                      println!("Percorsi salvati con successo in configuration.csv");
                      ctx.send_viewport_cmd(ViewportCommand::Close); // Chiude la finestra
                  }
              } else {
                  ui.label("Errore: seleziona entrambi i percorsi prima di continuare.");
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
