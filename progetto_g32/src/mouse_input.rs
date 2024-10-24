use rdev::{listen, Button, Event, EventType};
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;


#[derive(Debug, Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Default)]
struct Corners {
    point_a: bool,
    point_b: bool,
    point_c: bool,
    point_d: bool,
}

impl Corners {
    fn reset(&mut self) {
        self.point_a = false;
        self.point_b = false;
        self.point_c = false;
        self.point_d = false;
    }
}

lazy_static! {
    static ref SCREEN_DIMENSIONS: (f64, f64) = (1800.0, 1080.0); // Dimensioni dello schermo
    static ref CORNERS: Arc<Mutex<Corners>> = Arc::new(Mutex::new(Corners::default()));

    static ref START_POSITION: Mutex<Option<Position>> = Mutex::new(None);
    static ref END_POSITION: Mutex<Option<Position>> = Mutex::new(None);
    static ref CURRENT_POSITION: Mutex<Option<Position>> = Mutex::new(None);
    static ref IS_DRAWING: Mutex<bool> = Mutex::new(false);
}

// Questa è la funzione che gestisce il segno meno e ora la eseguiamo in un thread separato.
pub fn track_minus_sign(event: Event) {
    match event.event_type {
        EventType::MouseMove { x, y } => {
            // Aggiorna la posizione corrente del mouse durante il movimento
            *CURRENT_POSITION.lock().unwrap() = Some(Position { x, y });

        }
        EventType::ButtonPress(Button::Left) => {
            // Cattura la posizione iniziale al click del mouse
            if let Some(position) = *CURRENT_POSITION.lock().unwrap() {
                *START_POSITION.lock().unwrap() = Some(position);
                *IS_DRAWING.lock().unwrap() = true;
                println!("Inizio selezione: ({}, {})", position.x, position.y);
            }
        }
        EventType::ButtonRelease(Button::Left) => {
            // Cattura la posizione finale al rilascio del mouse
            if let Some(position) = *CURRENT_POSITION.lock().unwrap() {
                *END_POSITION.lock().unwrap() = Some(position);
                *IS_DRAWING.lock().unwrap() = false;
                println!("Fine selezione: ({}, {})", position.x, position.y);

                // Controlla se il segno meno è stato tracciato
                if let (Some(start), Some(end)) = (*START_POSITION.lock().unwrap(), *END_POSITION.lock().unwrap()) {
                    if is_minus_sign(start.x, start.y, end.x, end.y) {
                        println!("Segno meno tracciato correttamente!");
                    } else {
                        println!("Il segno tracciato non è un meno.");
                    }
                }
            }
        }
        _ => (),
    }
}

// Funzione per controllare se il movimento del mouse è un segno meno
fn is_minus_sign(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
    // Tolleranza per la deviazione verticale
    let vertical_tolerance = 10.0; // Permette un po' di movimento verticale
    let min_horizontal_distance = 50.0; // Distanza minima per considerarlo un segno meno

    let horizontal_distance = (x2 - x1).abs();
    let vertical_distance = (y2 - y1).abs();

    // Controlla se il movimento è principalmente orizzontale e abbastanza lungo
    horizontal_distance >= min_horizontal_distance && vertical_distance <= vertical_tolerance
}

// Questa è la funzione principale che gestisce gli eventi
fn handle_event(event: Event) {
    let (screen_width, screen_height) = *SCREEN_DIMENSIONS;
    let mut corners = CORNERS.lock().unwrap(); // Acquisiamo il lock sul Mutex
    let mut confirm_state = false;

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
                    confirm_state = true;
                    corners.reset(); // Reset dello stato dei vertici
                }
            }
        }
        _ => {}
    }

    // Se la sequenza è completata, avvia il tracking del segno meno in un nuovo thread
    if confirm_state {
        thread::spawn(|| {
            if let Err(err) = listen(track_minus_sign) {
                eprintln!("Errore durante il tracciamento del segno meno: {:?}", err);
            }
        });
    }
}

pub fn main() {
    // Esempio di ascolto eventi per il mouse
    if let Err(err) = listen(handle_event) {
        eprintln!("Errore nell'ascolto degli eventi: {:?}", err);
    }
}
