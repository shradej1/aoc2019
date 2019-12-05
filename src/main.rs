#![deny(unused_must_use)]

mod day1;
mod day2;
mod day3;

use day2::IntCodeProgram;

fn main() {
    println!(
        "Day 1: The Tyranny of the Rocket Equation: {:?}",
        day1::compute_fuel_requirement(&day1::get_modules())
    );

    // compute gravity assist parameters
    {
        let part2_ans = day2::get_gravity_assist_program()
            .search_for_output(19690720)
            .unwrap();

        println!(
            "Day 2: 1202 Program Alarm: {}, {}",
            day2::get_gravity_assist_program()
                .execute_with_args(12, 2)
                .unwrap(),
            100 * part2_ans.0 + part2_ans.1
        );
    }
}
