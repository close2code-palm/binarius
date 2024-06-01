use crate::file_ops::set_file_for_fan;
#[cfg(target_os = "linux")]
use crate::file_ops::{clear_fan, get_fan, read_events, set_dir_for_fan};
use std::sync::Arc;

mod file_ops;

#[cfg(target_os = "linux")]
fn run() {
    println!("running...");
    let fan = get_fan();
    set_dir_for_fan(&fan, "/more_safe/".to_string());
    println!("dir set");
    set_file_for_fan(&fan, "/safe/shit.txt");
    println!("file set");
    let fan_arc = Arc::new(fan);
    let fac = fan_arc.clone();
    ctrlc::set_handler(move || {
        clear_fan(&fan_arc);
    })
    .unwrap();
    println!("cc");
    read_events(&fac);
}

#[cfg(target_os = "windows")]
fn run() {
    panic!("not implemented!")
}

fn main() {
    println!("Hello, world!");
    run();
}
