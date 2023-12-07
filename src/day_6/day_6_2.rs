use std::borrow::Borrow;
use std::io::BufRead;
use std::str::Split;
use env_logger::init_from_env;
use env_logger::TimestampPrecision::Seconds;
use crate::load_txt;
use log::{debug, info};
use regex::Regex;

#[derive(Debug)]
struct Engine {
    engine: Vec<Vec<char>>,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_6/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    //use regex parser to get the data
    // Time:      7  15   30

    let re_time = regex::Regex::new(r"Time:(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}").unwrap();
    let re_distance = regex::Regex::new(r"Distance:(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}(?:\s+(\d+)){0,1}").unwrap();

    let times = match_regex(lines.clone(), re_time);
    let distances = match_regex(lines.clone(), re_distance);


    let mut time_str = String::new();
    for time in times{
        time_str.push_str(time.to_string().as_str());
    }
    let final_time = vec![time_str.parse::<usize>().unwrap()];

    let mut distance_str = String::new();
    for distance in distances{
        distance_str.push_str(distance.to_string().as_str());
    }
    let final_distance = vec![distance_str.parse::<usize>().unwrap()];

    debug!("Final time: {:?}", final_time);
    debug!("Final distance: {:?}", final_distance);

    let amount_of_ways_to_win = part_2(final_time, final_distance);


    // multiply all the values in the vector
    let total_ways_to_win = amount_of_ways_to_win.iter().fold(1, |a, b| a * b);



    info!("Total ways to win: {:?}", total_ways_to_win);
}

fn part_2(time: Vec<usize>, distance: Vec<usize>) -> Vec<usize> {
    let mut amount_of_ways_to_win: Vec<usize> = Vec::new();

    for i in 0..time.len() {
        debug!("Time: {:?} Distance: {:?}", time[i], distance[i]);
        let mut race_option_counter = 0;
        for load_time in 0..time[i] {
            let distance_traveled = (load_time * (time[i] - load_time));
            if distance_traveled > distance[i] {
                race_option_counter += 1;
            }
        }
        amount_of_ways_to_win.push(race_option_counter)
    }
    amount_of_ways_to_win
}

fn match_regex(lines: Split<&str>, re: Regex) -> Vec<usize> {
    let mut vector: Vec<usize> = Vec::new();
    // match and find regex to lines
    for line in lines {
        let caps = re.captures(line);
        match caps {
            Some(caps) => {
                // put the digits into a int variable
                for cap in caps.iter().skip(1) {
                    if cap.is_some() {
                        vector.push(cap.unwrap().as_str().parse::<usize>().unwrap());
                    }
                }
            }
            None => println!("No match"),
        }
    }
    debug!("Regex vector: {:?}", vector);
    vector
}
