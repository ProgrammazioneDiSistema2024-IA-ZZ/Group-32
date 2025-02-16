use std::path::Path;
use std::sync::mpsc::Receiver;
use egui::ViewportBuilder;

use crate::backup::backup;
use crate::main_configuration::{SOURCE_PATH, DESTINATION_PATH}; // Import delle variabili globali

pub fn run(rx: Receiver<()>) {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([350f32, 250f32]), // Dimensioni iniziali migliorate
        ..Default::default()
    };

    eframe::run_native(
        "Conferma Backup",
        options,
        Box::new(move |_cc| Ok(Box::new(ConfirmationApp { rx }))),
    ).expect("Errore nell'esecuzione della finestra di conferma");
}

struct ConfirmationApp {
    rx: Receiver<()>,
}

impl eframe::App for ConfirmationApp {
  fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
          ui.vertical_centered(|ui| {
              ui.add_space(20.0); // Spaziatura iniziale per centratura verticale

              ui.heading("Conferma Backup");
              ui.add_space(10.0); // Spaziatura tra titolo e testo

              ui.label("Sei sicuro di voler confermare il backup?");
              ui.add_space(30.0); // Spaziatura tra il testo e i pulsanti

              // Centrare i pulsanti orizzontalmente rispetto alla larghezza della finestra
              let button_width = 100.0;
              let spacing_between_buttons = 20.0;
              let total_buttons_width = 2.0 * button_width + spacing_between_buttons;
              let left_padding = (350.0 - total_buttons_width) / 2.0;

              ui.horizontal(|ui| {
                  ui.add_space(left_padding); // Padding iniziale per centrare i pulsanti

                  if ui
                      .add_sized([button_width, 40.0], egui::Button::new("Conferma").fill(egui::Color32::DARK_GREEN))
                      .clicked()
                  {
                      println!("Backup confermato.");
                      let source = Path::new(&*SOURCE_PATH);
                      let destination = Path::new(&*DESTINATION_PATH);
                      let file_types = vec!["txt", "jpg", "png"];

                      match backup(source, destination, file_types) {
                          Ok(_) => println!("Backup eseguito con successo!"),
                          Err(e) => eprintln!("Errore durante il backup: {:?}", e),
                      }
                      std::process::exit(0);
                  }

                  if ui
                      .add_sized([button_width, 40.0], egui::Button::new("Annulla").fill(egui::Color32::RED))
                      .clicked()
                  {
                      println!("Backup annullato.");
                      std::process::exit(0);
                  }
              });
          });
      });

      // Controlla se ci sono messaggi dal ricevitore
      if let Ok(_) = self.rx.try_recv() {
          println!("Finestra di conferma ricevuta.");
      }
  }
}
