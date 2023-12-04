use std::borrow::Borrow;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug)]
struct Engine {
    engine: Vec<Vec<char>>,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_3/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");
}
