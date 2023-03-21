use rust_gpiozero::*;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args[1].as_str() {
        "1" | "2" => println!("Valid"),
        _ => {println!("Invalid doornum"); return},
    }

    let doornum: u8 = match args[1].parse().unwrap() {
        1 => 12,
        2 => 16,
        _ => 6,
    };

    let led = LED::new(doornum);
    led.off();
    println!("{doornum} on");
    thread::sleep(Duration::from_millis(250));
    led.on();
    println!("{doornum} off");
}
