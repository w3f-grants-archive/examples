use std::{thread, time::Duration};

fn main() {
    thread::sleep(Duration::from_secs(20));
    println!("Hello, world!");
}
