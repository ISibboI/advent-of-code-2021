use clap::Parser;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

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
    day_4::problem_1();
    day_4::problem_2();
    day_5::problem_1();
    day_5::problem_2();
    day_6::problem_1();
    day_6::problem_2();
    day_7::problem_1();
    day_7::problem_2();
    day_8::problem_1();
    day_8::problem_2();
    day_9::problem_1();
    day_9::problem_2();
    day_10::problem_1();
    day_10::problem_2();
    day_11::problem_1();
    day_11::problem_2();
    day_12::problem_1();
    day_12::problem_2();
}
