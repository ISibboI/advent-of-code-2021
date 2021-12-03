use clap::Parser;

mod day_1;
mod day_2;
mod day_3;

#[derive(Parser)]
enum Command {
    Day1_1,
}

fn main() {
    /*match Command::parse() {
        Command::Day1_1 => day_1::problem_1(),
    }*/

    day_1::problem_1();
    day_1::problem_2();
    day_2::problem_1();
    day_2::problem_2();
    day_3::problem_1();
    day_3::problem_2();
}
