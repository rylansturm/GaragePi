use rust_gpiozero::*;
use std::thread;
use std::time::Duration;
use std::env;

const DOOR1RELAY: u8 = 22;
const DOOR1SENSOR: u8 = 27;
const DOOR2RELAY: u8 = 23;
const DOOR2SENSOR: u8 = 17;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if !check_args(&args) {
    } else {
        let command = &args[1];
        let door = &args[2];

        match command.as_str() {
            "open" => open(door),
            "close" => close(door),
            "check" => check(door),
            _ => println!("invalid command \"{}\".", command.as_str()),
        }
/*
        let doornum: u8 = match &args[1] {
            "1" => DOOR1RELAY,
            "2" => DOOR2RELAY,
            _ => 6,
        };

        let led = LED::new(doornum);
        led.off();
        println!("{doornum} on");
        thread::sleep(Duration::from_millis(250));
        led.on();
        println!("{doornum} off");
*/
    }
}

fn check_args(args: &Vec<String>) -> bool {
    true
}

fn open(door: &String) {
    if !status(door) {
        println!("opening door {}", door);
        press(&door);
    } else {
        println!("door {} is already open", door);
    }
}

fn close(door: &String) {
    if status(door) {
        println!("closing door {}", door);
        press(&door);
    } else {
        println!("door {} is already closed", door);
    }
}

fn check(door: &String) {
    let status: String = match status(door) {
        false => String::from("closed"),
        true => String::from("open"),
    };
    println!("door {} is {}", door, status);
}

fn press(door: &String) {
    let doorpin: u8 = match door.as_str() {
        "1" => DOOR1RELAY,
        "2" => DOOR2RELAY,
        _ => 0,
    };
    let door = LED::new(doorpin);
    door.on();
    thread::sleep(Duration::from_millis(250));
    door.off();
}

fn status(door: &String) -> bool {
    let doorpin: u8 = match door.as_str() {
        "1" => DOOR1SENSOR,
        "2" => DOOR2SENSOR,
        _ => 0,
    };
    InputDevice::new(doorpin).value()
}
