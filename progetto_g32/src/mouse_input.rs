use winit::window::Fullscreen;
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Definizione per l'area di selezione del rettangolo
pub struct SelectionArea {
    pub punto_a: bool,
    pub punto_b: bool,
    pub punto_c: bool,
    pub punto_d: bool,
}

impl SelectionArea {
    pub fn new() -> Self {
        Self {
            punto_a: false,
            punto_b: false,
            punto_c: false,
            punto_d: false,
        }
    }
}

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
}

enum AppState {
    Selection,
    Confirmation,
}

pub fn start_mouse_tracking() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_fullscreen(Some(Fullscreen::Borderless(None))) // Finestra a schermo intero
        .with_visible(true)
        .with_title("Backup Tool")
        .build(&event_loop)
        .unwrap();

    let mut selection_area = SelectionArea::new();
    let mut confirmation_area = ConfirmationArea::new();
    let mut app_state = AppState::Selection;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match app_state {
            AppState::Selection => {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        // Rileva il movimento del mouse per tracciare il rettangolo
                        WindowEvent::CursorMoved { position, .. } => {
                            if position.x >= 0.0 && position.x <= 20.0 && position.y >= 0.0 && position.y <= 20.0 {
                                println!("PUNTO A TROVATO");
                                selection_area.punto_a = true;
                            }
                            if position.x >= 0.0 && position.x <= 20.0 && position.y >= 2060.0 && position.y <= 2080.0 && selection_area.punto_a {
                                println!("PUNTO B TROVATO");
                                selection_area.punto_b = true;
                            }
                            if position.x >= 3580.0 && position.x <= 3600.0 && position.y >= 2060.0 && position.y <= 2080.0 && selection_area.punto_b {
                                println!("PUNTO C TROVATO");
                                selection_area.punto_c = true;
                            }
                            if position.x >= 3580.0 && position.x <= 3600.0 && position.y >= 0.0 && position.y <= 20.0 && selection_area.punto_c {
                                println!("PUNTO D TROVATO");
                                selection_area.punto_d = true;
                                println!("Passaggio alla fase di conferma");
                                app_state = AppState::Confirmation; // Passa alla fase di conferma
                            }
                            println!("Coordinate correnti: ({}, {})", position.x, position.y);
                        }
                        _ => {}
                    }
                }
            }
            AppState::Confirmation => {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        // Rileva il primo clic del mouse e imposta il primo vertice del rettangolo
                        WindowEvent::MouseInput {
                            state: ElementState::Pressed,
                            button: MouseButton::Left,
                            ..
                        } => {
                            if !confirmation_area.selecting {
                                confirmation_area.selecting = true;
                                println!("Mouse Pressed: Inizio selezione area rettangolare");
                                confirmation_area.start_x = confirmation_area.end_x;
                                confirmation_area.start_y = confirmation_area.end_y;
                            }
                        }

                        // Rileva il movimento del mouse per tracciare il rettangolo
                        WindowEvent::CursorMoved { position, .. } => {
                            if confirmation_area.selecting {
                                confirmation_area.end_x = position.x;
                                confirmation_area.end_y = position.y;
                                println!(
                                    "Selezione in corso: ({}, {}) -> ({}, {})",
                                    confirmation_area.start_x, confirmation_area.start_y,
                                    confirmation_area.end_x, confirmation_area.end_y
                                );
                            }
                        }

                        // Rileva il rilascio del mouse per confermare l'area selezionata
                        WindowEvent::MouseInput {
                            state: ElementState::Released,
                            button: MouseButton::Left,
                            ..
                        } => {
                            if confirmation_area.selecting {
                                confirmation_area.selecting = false;
                                println!(
                                    "Mouse Released: Area selezionata da ({}, {}) a ({}, {})",
                                    confirmation_area.start_x, confirmation_area.start_y,
                                    confirmation_area.end_x, confirmation_area.end_y
                                );

                                // Qui puoi procedere con il comando di backup
                                println!("Backup avviato!");
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    });
}
