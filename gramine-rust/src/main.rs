use std::{thread, time::Duration};

fn main() {
    thread::sleep(Duration::from_secs(10));
    println!("Hello, world!");
}
