use eframe::egui::{self, CentralPanel, ComboBox};
use std::fs;
use std::path::Path;
use std::io::Write;
use eframe::Frame;
use egui::{Align, Color32, Context, Layout, RichText, ViewportCommand, Window};
use rfd::FileDialog;

// Struttura del Config per toml
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Config {
    source_path: String,
    destination_path: String,
    backup_type: String,
    extensions_to_backup: Vec<String>,
}

// La GUI dell'applicazione
#[derive(Default)]
struct ConfigWindow {
    source_path: String,
    destination_path: String,
    backup_type: String,
    extensions_to_backup: String, // Le estensioni sono inserite come stringa, verranno separate dopo
}

impl ConfigWindow {
    // Metodo per salvare il file di configurazione
    fn save_config(&self) {
        let config = Config {
            source_path: self.source_path.clone(),
            destination_path: self.destination_path.clone(),
            backup_type: self.backup_type.clone(),
            extensions_to_backup: self.extensions_to_backup
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        };
        let toml_str = toml::to_string(&config).unwrap();
        let mut file = fs::File::create("config.toml").unwrap();
        file.write_all(toml_str.as_bytes()).unwrap();
    }

    // Method to open file dialog and select directory
    fn select_directory() -> Option<String> {
        FileDialog::new()
            .pick_folder()  // Opens folder dialog
            .map(|path| path.display().to_string())  // Converts selected path to string
    }
}

impl eframe::App for ConfigWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {

            // Set spacing and styling globally
            let spacing = ui.spacing_mut();
            spacing.item_spacing = egui::Vec2::new(5.0, 7.0); // Horizontal and vertical spacing between elements
            spacing.text_edit_width = 300.0; // Increase text field width

            ui.heading("Backup Configuration");

            ui.label("Source Path:");

            ui.horizontal(|ui| {
                // Campo per selezionare il percorso sorgente
                ui.text_edit_singleline(&mut self.source_path);

                // Button to open folder dialog
                if ui.button("...").clicked() {
                    if let Some(path) = ConfigWindow::select_directory() {
                        self.source_path = path;
                    }
                }
            });

            ui.label("Destination Path:");

            ui.horizontal(|ui| {
                // Campo per selezionare il percorso destinazione
                let input = ui.text_edit_singleline(&mut self.destination_path);

                if ui.button("...").clicked() {
                    if let Some(path) = ConfigWindow::select_directory(){
                        self.destination_path = path;
                    }
                }
            });

            // ComboBox per scegliere il tipo di backup
            ui.label("Backup Type:");

            ComboBox::from_label("")
                .selected_text(&self.backup_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.backup_type, "full-disk".to_string(), "Full Disk");
                    ui.selectable_value(&mut self.backup_type, "directory".to_string(), "Directory");
                    ui.selectable_value(&mut self.backup_type, "selective".to_string(), "Selective");
                });

            // Campo per inserire le estensioni da includere nel backup
            ui.label("File Extensions (comma separated):");

            ui.text_edit_singleline(&mut self.extensions_to_backup);

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let save_button_color = Color32::from_rgb(100, 250, 100); // Custom button color

                if ui.add(egui::Button::new("Save and Exit").fill(save_button_color)).clicked() {
                    self.save_config();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });
    }
}

// Funzione per avviare la GUI solo se config.toml non esiste
pub fn show_config_gui() -> Result<(), eframe::Error> {
    if !Path::new("config.toml").exists() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([350f32, 250f32]),
            ..Default::default()
        };
        eframe::run_native(
            "Backup Configuration",
            options,
            Box::new(|_cc| Ok(Box::new(ConfigWindow::default()))),
        )
    } else {
        println!("Il file config.toml esiste già, la GUI non verrà mostrata.");
        Ok(())
    }
}

#[derive(Default)]
struct BackupWindow;

impl eframe::App for BackupWindow {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Aggiungiamo un pop-up al centro dello schermo
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Titolo del pop-up
                ui.heading("Do you want to proceed with backup?");
                ui.add_space(10.0);
                // Legenda con le istruzioni
                ui.label("1. Scorri verso destra per eseguire il backup").highlight();
                ui.label("2. Scorri verso l'alto per annullare il backup").highlight();
                ui.label("3. Scorri in diagonale nel lato opposto per riconfigurare il backup").highlight();
            });
        });

    }
}

// Funzione per mostrare la finestra di backup come pop-up
pub fn show_backup_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_always_on_top()
            .with_inner_size([500f32, 250f32])
            .with_decorations(true)
            .with_drag_and_drop(true),
        centered: true,
        ..Default::default()
    };

    // Avvia l'interfaccia grafica con la finestra di backup
    eframe::run_native(
        "Backup Confirmation",
        options,
        Box::new(|_cc| Ok(Box::new(BackupWindow))),
    )
}