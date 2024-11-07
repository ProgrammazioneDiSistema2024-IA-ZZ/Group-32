use service_manager::native_service_manager;

mod mouse_input;
mod audio;
mod setup;
mod uninstall;
mod cpu_logger;

fn main() {
    println!("avvio programma");
    mouse_input::main();
    //confirmation_main()
}

