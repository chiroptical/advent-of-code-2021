// mod day1;
// mod day2;
// mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod lib;
mod nalgebra_testing;

// This could be useful for day 5
fn safe_convert(x: &f64) -> Option<isize> {
    if x.fract() == 0.0 {
        Some(*x as isize)
    } else {
        None
    }
}

fn main() {
    // day1::run();
    // day2::run();
    // day3::run();
    // day4::run();
    // nalgebra_testing::run();
    day5::run();
    // day6::run();
    // day7::run();

    println!("{:?}", safe_convert(&1.4));
}
