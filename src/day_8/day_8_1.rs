use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::thread::current;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, error, info};


pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_8/data_example.txt");

    // split the string on newlines
    let mut lines = input.split("\n");

    let instructions: &str = lines.next().unwrap().trim();

    // skip the second line
    lines.next().unwrap();

    let mut hash_map: HashMap<&str, Vec<String>> = HashMap::new();

    let mut current_location: &str = "";
    for (i,line) in lines.into_iter().enumerate() {
        let mut split = line.split("=");
        let origin = split.next().unwrap().trim();
        if i == 0 {
            current_location = origin;
        }
        let destination_formatted = split.next().unwrap().replace(" ","").replace("(","").replace(")","");
        let destination = destination_formatted.trim().split(",").map(|s| s.to_string()).collect::<Vec<String>>();
        hash_map.insert(origin, destination.clone());
    }

    debug!("Hash_map: {:?}", hash_map);


    let mut steps = 0;
    let hash_map_visited: HashMap<&str, &str> = HashMap::new();


    current_location = "AAA";
    'a: loop{
        // check if current location is in hash_map_visited
        // if hash_map_visited.contains_key(current_location) {
        //     current_location = hash_map_visited.get(current_location).unwrap();
        //     steps += instructions.len()-1;
        //     info!("JUMPING");
        //     continue
        // }
        for char in instructions.chars() {
            match char {
                'R' => {
                    debug!("Char of commands: {:?}", char);
                    debug!("Current location: {:?}", current_location);
                    debug!("Next location: {:?}", hash_map.get(current_location).unwrap()[1]);
                    current_location = &*hash_map.get(current_location).unwrap()[1];
                    steps += 1;
                }
                'L' => {
                    debug!("Response: {:?}", char);
                    debug!("Current location: {:?}", current_location);
                    debug!("Next location: {:?}", hash_map.get(current_location).unwrap()[0]);
                    current_location = &*hash_map.get(current_location).unwrap()[0];
                    steps += 1;
                }
                _ => {
                    debug!("Response: {:?}", char);
                    error!("Error: {:?}", char)
                }
            }
            if current_location == "ZZZ" {
                info!("Current location finished: {:?}", current_location);
                break 'a;
            }
        }
        info!("Amount of steps in cycle: {:?}", steps);
    }

    info!("Amount of steps: {:?}", steps);
}
