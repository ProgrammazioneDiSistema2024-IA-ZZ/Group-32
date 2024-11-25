use std::process::Command;
use std::{env, thread};
use crate::cpu_logger::log_cpu_usage;
use auto_launch::{AutoLaunchBuilder};

mod backup;
mod mouse_input;
mod audio;
mod cpu_logger;
mod confirmation_window;
mod configuration_window;
mod main_configuration;

mod bin {
    mod setup;
}


fn main() {

    let log_thread = thread::spawn(|| {
        log_cpu_usage();
    });

    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    // println!("{}", wd);

    /* Autostart configuration */
    let program_path = wd.join("progetto_g32");
    println!("{}", program_path.to_str().unwrap());


    #[cfg(not(target_os = "macos"))]
    {
        let auto = AutoLaunchBuilder::new()
            .set_app_name("Group13")
            .set_app_path(&program_path.to_str().unwrap())
            .set_use_launch_agent(false)
            .build()
            .unwrap();


        auto.enable().unwrap();
        println!("Autostart enabled: {}", auto.is_enabled().unwrap());
    }

    #[cfg(target_os = "macos")]
    {
        let _ = AutoLaunchBuilder::new()
            .set_app_name("Group13")
            .set_app_path(&program_path.to_str().unwrap())
            .set_use_launch_agent(false)
            .build()
            .unwrap().enable();

        Command::new("osascript")
            .arg("-e")
            .arg("tell application \"Terminal\" to set visible of front window to false")
            .output()
            .expect("Failed to hide terminal");
    }


    println!("avvio programma!");
    main_configuration::main();


    // Attende la terminazione del thread di logging (se necessario)
    log_thread.join().expect("Errore nel thread di logging");
}
