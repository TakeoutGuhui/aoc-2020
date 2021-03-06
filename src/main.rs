use std::time::Instant;

#[macro_use]
extern crate lazy_static;
mod utils;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_7;

fn main() {
    let total_time = Instant::now();
    // TODO Do it some other way
    println!("Day 1 part 1:");
    day_1::part_1();
    println!("Day 1 part 2:");
    day_1::part_2();
    println!("Day 2 part 1:");
    day_2::part_1();
    println!("Day 2 part 2:");
    day_2::part_2();
    println!("Day 3 part 1:");
    day_3::part_1();
    println!("Day 3 part 2:");
    day_3::part_2();
    println!("Day 4 part 1:");
    day_4::part_1();
    println!("Day 7 part 1:");
    day_7::part_1_and_2();
    println!("Total time: {}", total_time.elapsed().as_millis());
}
