use crate::Example_reddit::part_two;
use log::info;
use crate::Example_reddit::{parse_input, part_one};

mod load_txt;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod Example_reddit;

fn main() {
    // Initialize the logger
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    // day_1::day_1_1::run();
    // day_1::day_1_2::run();

    // day_2::day_2_1::run();
    // day_2::day_2_2::run();

    // day_3::day_3_2::run();
    // day_3::day_3_1::run();

    // day_4::day_4_1::run();
    // day_4::day_4_2::run();

    // unsafe { day_5::day_5_1::run(); }
    // unsafe { day_5::day_5_2::run(); }

    // day_6::day_6_1::run();
    // day_6::day_6_2::run();

    // day_7::day_7_1::run();
    // day_7::day_7_2::run();

    // day_8::day_8_1::run();
    // day_8::day_8_2::run();

    // day_9::day_9_1::run();
    // day_9::day_9_2::run();

    // day_10::day_10_1::run();
    day_10::day_10_2::run();


    // let input = load_txt::load_txt_file("src/day_5/data_example.txt");
    // let input_parse = parse_input(&input);
    // info!("Response: {:?}", part_one(&input_parse));
    // info!("Response: {:?}", part_two(&input_parse));







}
