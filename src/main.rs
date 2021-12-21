use clap::Parser;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
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
    day_13::problem_1();
    day_13::problem_2();
    day_14::problem_1();
    day_14::problem_2();
    day_15::problem_1();
    day_15::problem_2();
    day_16::problem_1();
    day_16::problem_2();
    day_17::problem_1();
    day_17::problem_2();
    day_18::problem_1();
    day_18::problem_2();
    day_19::problem_1();
    day_19::problem_2();
    day_20::problem_1();
    day_20::problem_2();
    day_21::problem_1();
    day_21::problem_2();
    day_22::problem_1();
    day_22::problem_2();
    day_23::problem_1();
    day_23::problem_2();
    day_24::problem_1();
    day_24::problem_2();
    day_25::problem_1();
    day_25::problem_2();
}
