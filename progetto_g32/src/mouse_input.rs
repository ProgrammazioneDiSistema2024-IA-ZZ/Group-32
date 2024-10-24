use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use crate::mouse_input_confirmation::{start_mouse_tracking};

// Struttura per mantenere lo stato del rilevamento dei vertici
struct ScreenCorners {
    point_a: bool,
    point_b: bool,
    point_c: bool,
    point_d: bool,
}

impl ScreenCorners {
    fn new() -> Self {
        ScreenCorners {
            point_a: false,
            point_b: false,
            point_c: false,
            point_d: false,
        }
    }

    // Reset dei vertici dopo il completamento del back-up
    fn reset(&mut self) {
        self.point_a = false;
        self.point_b = false;
        self.point_c = false;
        self.point_d = false;
    }
}

struct ConfirmationState {
    is_confirming: bool,
    start_x: f64,
    start_y: f64,
    end_x: f64,
}

impl ConfirmationState {
    pub fn new() -> Self {
        Self {
            is_confirming: false,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
        }
    }

    pub fn check_minus_sign(&self) -> bool {
        let threshold_distance = 100.0;
        let y_tolerance = 20.0;

        (self.end_x - self.start_x).abs() >= threshold_distance
            && (self.start_y - self.end_x).abs() <= y_tolerance
    }
}


// Variabili globali con Mutex per gestire l'accesso condiviso
lazy_static::lazy_static! {
    static ref CORNERS: Mutex<ScreenCorners> = Mutex::new(ScreenCorners::new());
    static ref SCREEN_DIMENSIONS: (f64, f64) = (1800.0, 1080.0);
}

pub fn main() {
    // Avvia l'ascolto degli eventi del mouse
    if let Err(error) = listen(handle_event) {
        println!("Errore durante l'ascolto degli eventi: {:?}", error);
    }
}

// Funzione che gestisce gli eventi del mouse
fn handle_event(event: Event) {
    let (screen_width, screen_height) = *SCREEN_DIMENSIONS;
    let mut corners = CORNERS.lock().unwrap(); // Acquisiamo il lock sul Mutex

    match event.event_type {
        EventType::MouseMove { x, y } => {
            let tolerance = 20.0;

            // Controlla il primo vertice in alto a sinistra (A)
            if (x <= tolerance && y <= tolerance) && !corners.point_a {
                println!("PUNTO A (in alto a sinistra) trovato");
                corners.point_a = true;
            }

            // Controlla il secondo vertice in basso a sinistra (B)
            if (x <= tolerance && y >= screen_height - tolerance) && corners.point_a && !corners.point_b {
                println!("PUNTO B (in basso a sinistra) trovato");
                corners.point_b = true;
            }

            // Controlla il terzo vertice in basso a destra (C)
            if (x >= screen_width - tolerance && y >= screen_height - tolerance) && corners.point_b && !corners.point_c {
                println!("PUNTO C (in basso a destra) trovato");
                corners.point_c = true;
            }

            // Controlla l'ultimo vertice in alto a destra (D)
            if (x >= screen_width - tolerance && y <= tolerance) && corners.point_c && !corners.point_d {
                println!("PUNTO D (in alto a destra) trovato");
                corners.point_d = true;

                // Se tutti i punti sono stati trovati, avvia il backup
                if corners.point_a && corners.point_b && corners.point_c && corners.point_d {
                    println!("Sequenza completata.");
                    start_mouse_tracking();
                    corners.reset(); // Reset dello stato dei vertici
                }
            }
        }
        _ => {}
    }
}

// Funzione di esempio per eseguire il backup
pub fn backup_procedure() {
    println!("Eseguo il backup...");
    // Logica di backup qui
}


