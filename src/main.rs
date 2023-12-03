mod load_txt;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    // Initialize the logger
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();


    // day_1::day_1_2::run();
    // day_2::day_2_2::run();
    day_3::day_3_2::run();
}
