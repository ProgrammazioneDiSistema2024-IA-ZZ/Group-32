use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent, ElementState, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct ConfirmationArea {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub selecting: bool,
}

impl ConfirmationArea {
    pub fn new() -> Self {
        Self {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            selecting: false,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        (self.end_y - self.start_y).abs() < 10.0 && (self.end_x - self.start_x).abs() > 50.0
    }
}

pub fn backup_confirmation() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(true)
        .with_title("Confirm Backup - Traccia il segno meno")
        .build(&event_loop)
        .unwrap();

    let mut confirmation_area = ConfirmationArea::new();
    let mut cursor_position = PhysicalPosition::new(0.0, 0.0); // Cambiato a PhysicalPosition
    let mut completed = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                // Rileva il movimento del cursore
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_position = position; // Aggiornamento posizione
                    println!(
                        "Cursore mosso: Posizione attuale ({:.2}, {:.2})",
                        cursor_position.x, cursor_position.y
                    );
                }
                // Rileva il primo clic del mouse
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    if !confirmation_area.selecting {
                        confirmation_area.selecting = true;
                        confirmation_area.start_x = cursor_position.x;
                        confirmation_area.start_y = cursor_position.y;
                        println!(
                            "Mouse cliccato in: ({:.2}, {:.2}) - Inizio tracciamento",
                            confirmation_area.start_x, confirmation_area.start_y
                        );
                    }
                }
                // Rileva il rilascio del mouse
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    ..
                } => {
                    if confirmation_area.selecting {
                        confirmation_area.end_x = cursor_position.x;
                        confirmation_area.end_y = cursor_position.y;
                        confirmation_area.selecting = false;

                        println!(
                            "Mouse rilasciato in: ({:.2}, {:.2}) - Fine tracciamento",
                            confirmation_area.end_x, confirmation_area.end_y
                        );

                        if confirmation_area.is_horizontal() {
                            println!("Segno meno tracciato correttamente!");
                            completed = true;
                        } else {
                            println!("Segno meno non corretto, riprova.");
                        }

                        if completed {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
