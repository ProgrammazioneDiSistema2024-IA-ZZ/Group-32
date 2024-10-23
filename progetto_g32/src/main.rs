mod mouse_input;

fn main() {
    // Inizializza l'input del mouse e il ciclo degli eventi
    println!("Avvio del tool di backup...");
    mouse_input::start_mouse_tracking();
}
