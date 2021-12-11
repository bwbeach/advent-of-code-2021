use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: advent <dayNumber>");
        std::process::exit(1);
    }
    let day_number: i32 = args[1].parse().unwrap();
    println!("Day {:?}", day_number);
}
