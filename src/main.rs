#[cfg(target_os = "linux")]
use crate::file_ops::{clear_fan, get_fan, read_events, set_dir_for_fan};

mod file_ops;

#[cfg(target_os = "linux")]
fn run() {
    let fan = get_fan();
    set_dir_for_fan(&fan, "/safe/".to_string());
    ctrlc::set_handler(|| {
        clear_fan(&fan);
    })
    .unwrap();
    read_events(&fan);
}

#[cfg(target_os = "windows")]
fn run() {
    panic!("not implemented!")
}

fn main() {
    println!("Hello, world!");
    run();
}
