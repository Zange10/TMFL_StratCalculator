// Calculate possible strategies for races of the Trackmania Formula League TMFL

use std::io;

#[allow(dead_code)]
struct Stop {
    time: f64,
    fuel: f64,
}

#[allow(dead_code)]
struct Stint {
    laps: i32,
    init_fuel: f64,
    end_fuel: f64,
    tyre_cond: f64,
}

//const refuel_rate:f64 = 4.0;
//const pitlane_time:f64 = 20.0;



fn main() {
    let race_laps = get_input_i32("Race Laps: ");
    println!("{}", race_laps);
}

fn get_input_i32(msg:&str) -> i32 {
    println!("{}", msg);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x:i32 = input_line.trim().parse().expect("Input not an integer");
    return x;
}