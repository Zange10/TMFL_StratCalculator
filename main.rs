// Calculate possible strategies for races of the Trackmania Formula League TMFL

use std::*;

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

struct Strategy {
    stops: i32,
    stint_length: [i32;7],
    pit_laps: [i32;7],
    max_tyre_wear: f64,
    max_fuel_cons: f64,
}

//const pitlane_time:f64 = 30.0;
const MIN_TYRE_COND:f64 = 5.0;
const MIN_FUEL_LOAD:f64 = 2.0;

impl Copy for Strategy {}

impl Clone for Strategy {
    fn clone(&self) -> Strategy {
        Strategy{stops: self.stops, stint_length: self.stint_length, pit_laps: self.pit_laps, max_tyre_wear: self.max_tyre_wear, max_fuel_cons:self.max_fuel_cons}
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "################\n{}-Stop Strategy:\nStints: {}\nPitting laps: {}\n\nMax Tyre Wear: \t\t{:.2} per lap\nMax Fuel Consumption: \t{:.2} per lap\n--------------\n", self.stops, array_to_string(self.stint_length), array_to_string(self.pit_laps), self.max_tyre_wear, self.max_fuel_cons)
    }
}


impl Strategy {
    fn new() -> Strategy {
        return Strategy {stops: 0, stint_length: [0; 7], pit_laps: [0; 7], max_tyre_wear: 0.0, max_fuel_cons:0.0};
    }


    fn create(mut laps:i32, stops:i32) -> Strategy {
        let mut stint_length: [i32; 7] = [0;7];
        let mut pit_laps: [i32; 7] = [0;7];
        let mut max_tyre_wear: f64 = 0.0;
        let mut max_fuel_cons: f64 = 0.0;
        let mut stints = stops+1;
    
        for i in 0..=stops {
            stint_length[i as usize] = (laps as f32/stints as f32).ceil() as i32;
            stints-=1;
            laps -= stint_length[i as usize];
            if i == 0 {
                max_tyre_wear = (100.0-MIN_TYRE_COND)/stint_length[0] as f64;
                max_fuel_cons = (100.0-MIN_FUEL_LOAD)/stint_length[0] as f64;
                pit_laps[i as usize] = stint_length[i as usize];
            } else if i < stops{
                pit_laps[i as usize] = stint_length[i as usize] + pit_laps[(i-1) as usize];
            }

        }

        Strategy {stops, stint_length, pit_laps, max_tyre_wear, max_fuel_cons}
    }
}






fn main() {
    let race_laps = get_input_i32("Race Laps: ");
    
    let mut strats:[Strategy; 5] = [Strategy::new(); 5];

    for i in 2..=6 {
        strats[i-2] = Strategy::create(race_laps, i as i32);
    }

    println!("\n");
    for strat in strats.iter() {
        println!("{}", strat);
    }
}


fn get_input_i32(msg:&str) -> i32 {
    println!("{}", msg);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x:i32 = input_line.trim().parse().expect("Input not an integer");
    return x;
}

fn array_to_string(arr:[i32;7]) -> String {
    let mut string = String::new();
    let mut first = true;
    for n in arr {
        if n > 0 {
            if first {first = false;}
            else {string.push_str(", ");}
            string.push_str(&n.to_string());

        }
    }
    string
}