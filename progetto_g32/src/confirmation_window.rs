use std::sync::mpsc::Receiver;

pub fn run(rx: Receiver<()>) {
    eframe::run_native(
        "Conferma Backup",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| Box::new(ConfirmationApp { rx })),
    );
}

struct ConfirmationApp {
    rx: Receiver<()>,
}

impl eframe::App for ConfirmationApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Controlla se ci sono messaggi dal ricevitore
        if let Ok(_) = self.rx.try_recv() {
            // Mostra una finestra di dialogo
            eframe::egui::Window::new("Conferma Backup")
                .collapsible(false)
                .show(ctx, |ui| {
                    if ui.button("Conferma").clicked() {
                        println!("Backup confermato.");
                    }
                    if ui.button("Annulla").clicked() {
                        println!("Backup annullato.");
                    }
                });
        }
    }
}
