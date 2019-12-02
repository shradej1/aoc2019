mod day1;
mod day2;

fn main() {
    println!("Day 1: The Tyranny of the Rocket Equation");
    println!("{:?}", day1::compute_fuel_requirement(&day1::get_modules()));
}
