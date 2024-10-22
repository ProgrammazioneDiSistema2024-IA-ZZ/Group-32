mod mouse_input;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Creazione del ciclo di eventi
    let event_loop = EventLoop::new();

    // Creazione della finestra
    let window = WindowBuilder::new()
        .with_title("Selezione Area con il Mouse")
        .build(&event_loop)
        .unwrap();

    // Crea l'area di selezione
    let mut selection_area = mouse_input::SelectionArea::new();

    // Inizia il ciclo di gestione eventi
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            // Richiede il ridisegno della finestra
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // Chiudi la finestra quando viene richiesta la chiusura
                *control_flow = ControlFlow::Exit;
                println!("Evento: Chiusura della finestra richiesta");
            }

            Event::WindowEvent { event, .. } => {
                // Passa l'evento al gestore degli input del mouse
                mouse_input::handle_mouse_input(&event, &mut selection_area);
            }

            Event::RedrawRequested(_) => {
                // Qui puoi implementare la logica per ridisegnare la finestra e, ad esempio, mostrare il rettangolo di selezione
                if let Some(((x1, y1), (x2, y2))) = selection_area.get_rect() {
                    // Logica per disegnare il rettangolo sulla finestra
                    println!("Rettangolo selezionato: da ({}, {}) a ({}, {})", x1, y1, x2, y2);
                } else {
                    println!("Nessuna area selezionata al momento.");
                }
            }

            _ => {}
        }
    });
}
