// src/mouse_input.rs
use rdev::{listen, Button, Event, EventType};
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;
use scrap::{Capturer, Display}; // Importa le librerie necessarie da scrap
use crate::audio::play_sound;

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

lazy_static! {
    static ref SCREEN_DIMENSIONS: (f64, f64) = get_screen_dimensions();
}

pub fn track_minus_sign(event: Event) {
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
            if let Some(position) = *CURRENT_POSITION.lock().unwrap() {
                *START_POSITION.lock().unwrap() = Some(position);
                *IS_DRAWING.lock().unwrap() = true;
                println!("Inizio selezione: ({}, {})", position.x, position.y);
            }
        }
        EventType::ButtonRelease(Button::Left) => {
            if let Some(position) = *CURRENT_POSITION.lock().unwrap() {
                *END_POSITION.lock().unwrap() = Some(position);
                *IS_DRAWING.lock().unwrap() = false;
                println!("Fine selezione: ({}, {})", position.x, position.y);

                if let (Some(start), Some(end)) = (*START_POSITION.lock().unwrap(), *END_POSITION.lock().unwrap()) {
                    if is_minus_sign(start.x, start.y, end.x, end.y) {
                        println!("Segno meno tracciato correttamente!");
                        play_sound();
                    } else {
                        println!("Il segno tracciato non è un meno.");
                    }
                }
            }
        }
        _ => (),
    }
}

fn is_minus_sign(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
    let vertical_tolerance = 10.0;
    let min_horizontal_distance = 50.0;

    let horizontal_distance = (x2 - x1).abs();
    let vertical_distance = (y2 - y1).abs();

    horizontal_distance >= min_horizontal_distance && vertical_distance <= vertical_tolerance
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
                if let Err(err) = listen(track_minus_sign) {
                    eprintln!("Errore durante il tracciamento del segno meno: {:?}", err);
                }
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
