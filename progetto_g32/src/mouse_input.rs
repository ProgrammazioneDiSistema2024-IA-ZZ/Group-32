use std::path::Path;
// src/mouse_input.rs
use rdev::{listen, Button, Event, EventType};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use scrap::{Display}; // Importa le librerie necessarie da scrap
use crate::audio::play_sound;
use device_query::{DeviceQuery, DeviceState, MouseState};
use crate::backup::backup;

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
    static ref CORNERS: Arc<Mutex<Corners>> = Arc::new(Mutex::new(Corners::default()));
    static ref START_POSITION: Mutex<Option<Position>> = Mutex::new(None);
    static ref END_POSITION: Mutex<Option<Position>> = Mutex::new(None);
    static ref CURRENT_POSITION: Mutex<Option<Position>> = Mutex::new(None);
    static ref IS_DRAWING: Mutex<bool> = Mutex::new(false);
    static ref IS_TRACKING_MINUS: Mutex<bool> = Mutex::new(false);
    static ref LAST_EVENT: Mutex<Option<EventType>> = Mutex::new(None);
    static ref SCREEN_DIMENSIONS: (f64, f64) = get_screen_dimensions();
    static ref IS_RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

fn get_screen_dimensions() -> (f64, f64) {
    match Display::primary() {
        Ok(display) => {
            let width = display.width() as f64;
            let height = display.height() as f64;
            println!("Dimensioni del display: larghezza = {}, altezza = {}", width, height);
            (width, height)
        }
        Err(err) => {
            eprintln!("Errore nel recupero delle dimensioni dello schermo: {:?}", err);
            (0.0, 0.0) // Valori di fallback
        }
    }
}


// Funzione per tracciare il segno meno
fn track_minus_sign(event: Event) {
    let mut last_event = LAST_EVENT.lock().unwrap();
    if *last_event == Some(event.event_type.clone()) {
        return;
    }
    *last_event = Some(event.event_type.clone());

    match event.event_type {
        EventType::MouseMove { x, y } => {
            *CURRENT_POSITION.lock().unwrap() = Some(Position { x, y });
        }
        EventType::ButtonPress(Button::Left) => {
            let position = get_mouse_position();
            *CURRENT_POSITION.lock().unwrap() = Some(position);
            *START_POSITION.lock().unwrap() = Some(position);
            *IS_DRAWING.lock().unwrap() = true;
            println!("Inizio selezione: ({}, {})", position.x, position.y);

            // Avvia un thread per monitorare il movimento del cursore
            thread::spawn(move || {
                while *IS_DRAWING.lock().unwrap() {
                    let position = get_mouse_position();
                    *CURRENT_POSITION.lock().unwrap() = Some(position);
                    println!("Coordinate correnti monitorate manualmente: ({}, {})", position.x, position.y);
                    thread::sleep(Duration::from_millis(10)); // intervallo di controllo
                }
            });
        }
        EventType::ButtonRelease(Button::Left) => {
            *IS_DRAWING.lock().unwrap() = false;
            if let Some(position) = *CURRENT_POSITION.lock().unwrap() {
                println!("Fine selezione: ({}, {})", position.x, position.y);
                let Some(start) = *START_POSITION.lock().unwrap() else { todo!() };
                if is_minus_sign(start.x, start.y, position.x, position.y) {
                    println!("Segno meno tracciato correttamente!");
                    play_sound();

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
                } else {
                    println!("Il segno tracciato non è un meno.");
                }
            }
        }
        _ => (),
    }
}
// Funzione per ottenere la posizione del mouse usando device_query
fn get_mouse_position() -> Position {
    let device_state = DeviceState::new();
    let mouse_state: MouseState = device_state.get_mouse();

    Position{
        x: mouse_state.coords.0 as f64,
        y: mouse_state.coords.1 as f64,
    }
}
// Funzione per verificare se la selezione è un segno meno (orizzontale)
fn is_minus_sign(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
    let delta_x = (x2 - x1).abs();
    let delta_y = (y2 - y1).abs();
    delta_y < 10.0 && delta_x > 50.0
}


fn handle_event(event: Event) {
    let (screen_width, screen_height) = *SCREEN_DIMENSIONS;
    let mut corners = CORNERS.lock().unwrap();
    let mut confirm_state = false;
    match event.event_type {
        EventType::MouseMove { x, y } => {
            let tolerance = 30.0; // Tolleranza per migliorare la precisione

            // Angolo A (in alto a sinistra)
            if (x <= tolerance && y <= tolerance) && !corners.point_a {
                println!("PUNTO A trovato: x={}, y={}", x, y);
                corners.point_a = true;
            }

            // Angolo B (in basso a sinistra)
            if (x <= tolerance && y >= screen_height - tolerance) && corners.point_a && !corners.point_b {
                println!("PUNTO B trovato: x={}, y={}", x, y);
                corners.point_b = true;
            }

            // Angolo C (in basso a destra)
            if (x >= screen_width - tolerance && y >= screen_height - tolerance) && corners.point_b && !corners.point_c {
                println!("PUNTO C trovato: x={}, y={}", x, y);
                corners.point_c = true;
            }

            // Angolo D (in alto a destra)
            if (x >= screen_width - tolerance && y <= tolerance) && corners.point_c && !corners.point_d {
                println!("PUNTO D trovato: x={}, y={}", x, y);
                corners.point_d = true;

                if corners.point_a && corners.point_b && corners.point_c && corners.point_d {
                    println!("Sequenza completata.");
                    confirm_state = true;
                    corners.reset();
                }
            }
        }
        _ => {}
    }

    if confirm_state {
        let mut is_tracking = IS_TRACKING_MINUS.lock().unwrap();
        if !*is_tracking {
            *is_tracking = true;
            thread::spawn(|| {
                listen(move |event| {
                    track_minus_sign(event);
                }).unwrap();
                *IS_TRACKING_MINUS.lock().unwrap() = false;
            });
        }
    }
}

pub fn main() {
    if let Err(err) = listen(handle_event) {
        eprintln!("Errore nell'ascolto degli eventi: {:?}", err);
    }
}
