mod backup;
mod mouse_input;
mod audio;

use std::path::Path;
use std::fs;
use crate::backup::backup;

fn main() {
    println!("avvio programma!");
    mouse_input::main();
}
