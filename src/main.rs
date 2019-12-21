#![deny(unused_must_use)]

mod day1;
mod day2;
mod day3;
mod day4;
//mod day5; -> added to IntCodeProgram from day2
mod day6;

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

    {
        use day3::*;
        let (p1, p2) = get_paths();
        let intersections = intersect(&p1, &p2);
        let min_manhattan = compute_min_manhattan_distance(&intersections).unwrap();
        let min_steps = compute_min_total_steps(&intersections, &p1, &p2).unwrap();
        println!(
            "Day 3: Minimum manhattan distance: {}, minimum steps: {}",
            min_manhattan, min_steps
        );
    }

    {
        use day4::*;
        let (p1, p2) = password_search_size();
        println!(
            "Password search size with adjacency group >= 2: {}, == 2: {}",
            p1, p2
        );
    }
}
