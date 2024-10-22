use winit::event::{ElementState, MouseButton, WindowEvent};

/// Struttura per rappresentare l'area di selezione.
#[derive(Debug)]
pub struct SelectionArea {
    pub start_pos: Option<(f32, f32)>, // Posizione iniziale (al primo clic)
    pub end_pos: Option<(f32, f32)>,   // Posizione finale (al rilascio del mouse)
}

impl SelectionArea {
    /// Crea una nuova area di selezione.
    pub fn new() -> Self {
        SelectionArea {
            start_pos: None,
            end_pos: None,
        }
    }

    /// Calcola e restituisce il rettangolo di selezione.
    pub fn get_rect(&self) -> Option<((f32, f32), (f32, f32))> {
        if let (Some(start), Some(end)) = (self.start_pos, self.end_pos) {
            Some((start, end))
        } else {
            None
        }
    }

    /// Aggiunge l'input del mouse per il primo clic.
    pub fn start_selection(&mut self, x: f32, y: f32) {
        self.start_pos = Some((x, y));
        self.end_pos = None; // Reset dell'area finale
        println!("Mouse cliccato: posizione iniziale ({}, {})", x, y);
    }

    /// Traccia il rettangolo durante il trascinamento.
    pub fn update_selection(&mut self, x: f32, y: f32) {
        if let Some((start_x, start_y)) = self.start_pos {
            self.end_pos = Some((x, y));
            println!("Trascinamento in corso: da ({}, {}) a ({}, {})", start_x, start_y, x, y);
        }
    }

    /// Completa l'azione al rilascio del mouse.
    pub fn confirm_selection(&mut self, x: f32, y: f32) {
        if self.start_pos.is_some() {
            self.end_pos = Some((x, y)); // Fissa la posizione finale
            println!("Mouse rilasciato: selezione confermata da a ({}, {})", x, y);
        }
    }
}

/// Gestisce gli eventi del mouse per la selezione dell'area.
pub fn handle_mouse_input(event: &WindowEvent, selection_area: &mut SelectionArea) {
    match event {
        // Rileva il primo clic del mouse
        WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        } => {
            if let Some(position) = get_cursor_position(event) {
                println!("Evento: Mouse Pressed");
                selection_area.start_selection(position.0, position.1);
            }
        }

        // Tracciamento del rettangolo durante il trascinamento
        WindowEvent::CursorMoved { position, .. } => {
            println!("Evento: Mouse Moved to ({}, {})", position.x, position.y);
            selection_area.update_selection(position.x as f32, position.y as f32);
        }

        // Completa l'azione al rilascio del mouse
        WindowEvent::MouseInput {
            state: ElementState::Released,
            button: MouseButton::Left,
            ..
        } => {
            if let Some(position) = get_cursor_position(event) {
                println!("Evento: Mouse Released");
                selection_area.confirm_selection(position.0, position.1);
            }
        }

        _ => {}
    }
}

/// Ottiene la posizione corrente del cursore dal WindowEvent
fn get_cursor_position(event: &WindowEvent) -> Option<(f32, f32)> {
    if let WindowEvent::CursorMoved { position, .. } = event {
        Some((position.x as f32, position.y as f32))
    } else {
        None
    }
}
