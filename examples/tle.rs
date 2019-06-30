extern crate n2yo;

use std::env;

const USAGE : &str = "Usage: tle API_KEY NORAD_ID";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("{}", USAGE);
        std::process::exit(1);
    }

    let client = n2yo::Client::new(&args[1]);
    let result = client.tle(args[2].parse().unwrap()).unwrap();
    println!("Norad ID: {}", result.info.satid);
    println!("Satellite name: {}", result.info.satname);
    println!("TLE:\r\n{}", result.tle);
}
