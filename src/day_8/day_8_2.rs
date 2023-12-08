use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::thread::current;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, error, info};
use num_integer::lcm;


pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_8/data_final.txt");

    // split the string on newlines
    let mut lines = input.split("\n");

    let instructions: &str = lines.next().unwrap().trim();

    // skip the second line
    lines.next().unwrap();

    let mut hash_map: HashMap<&str, Vec<String>> = HashMap::new();

    let mut list_A_starts: Vec<&str> = Vec::new();
    let mut list_Z_ends: Vec<&str> = Vec::new();

    let mut current_location: &str = "";
    for (i,line) in lines.into_iter().enumerate() {
        let mut split = line.split("=");
        let origin = split.next().unwrap().trim();
        // compare third char of origin to A or Z
        if origin.chars().nth(2).unwrap() == 'A' {
            list_A_starts.push(origin);
        } else if origin.chars().nth(2).unwrap() == 'Z' {
            list_Z_ends.push(origin);
        }
        let destination_formatted = split.next().unwrap().replace(" ","").replace("(","").replace(")","");
        let destination = destination_formatted.trim().split(",").map(|s| s.to_string()).collect::<Vec<String>>();
        hash_map.insert(origin, destination.clone());
    }


    debug!("Hash_map: {:?}", hash_map);
    debug!("List A starts: {:?}", list_A_starts);
    debug!("List Z ends: {:?}", list_Z_ends);

    let mut steps = 0;
    let hash_map_visited: HashMap<&str, &str> = HashMap::new();


    let mut current_location_vec: Vec<&str> = Vec::new();
    for start in list_A_starts.clone() {
        current_location_vec.push(start);
    }
    // create a vector of steps for each A start
    let mut steps: Vec<usize> = vec![0; list_A_starts.len()];
    let mut final_steps: Vec<usize> = Vec::new();

    current_location = "AAA";
    'a: loop{
        for char in instructions.chars() {
            let mut all_z_locations = 0;
            let current_location_vec_clone = current_location_vec.clone();
            if current_location_vec.len() == 0 {
                info!("No more locations to check");
                break 'a;
            }
            current_location_vec.clear();
            let mut compensated_index = 0;
            for (mut i, location) in current_location_vec_clone.into_iter().enumerate(){
                if list_Z_ends.contains(&location) {
                    info!("Current location finished: {:?}", location);
                    final_steps.push(steps[i]);
                    // remove steps[i] from steps
                    steps.remove(i);
                    continue
                }
                match char {
                    'R' => {
                        debug!("Char of commands: {:?}", char);
                        debug!("Current location: {:?}", location);
                        debug!("Next location: {:?}", hash_map.get(location).unwrap()[1]);
                        current_location_vec.push(&*hash_map.get(location).unwrap()[1]);
                        steps[compensated_index] += 1;
                    }
                    'L' => {
                        debug!("Response: {:?}", char);
                        debug!("Current location: {:?}", location);
                        debug!("Next location: {:?}", hash_map.get(location).unwrap()[0]);
                        current_location_vec.push(&*hash_map.get(location).unwrap()[0]);
                        steps[compensated_index] += 1;
                    }
                    _ => {
                        debug!("Response: {:?}", char);
                        error!("Error: {:?}", char)
                    }
                }
                if list_Z_ends.contains(&location) {
                    info!("Current location finished: {:?}", location);
                    all_z_locations += 1;
                }
                compensated_index += 1;
            }
            if all_z_locations == list_A_starts.len() {
                debug!("All Z locations found");
                break 'a;
            }
        }
        info!("Amount of steps in cycle: {:?}", steps);
    }

    //find common multiple of all steps
    let mut common_multiple:u128 = final_steps[0] as u128;
    for step in final_steps.clone() {
        common_multiple = lcm(common_multiple, step as u128);
    }
    info!("Common multiple: {:?}", common_multiple);
    info!("Steps: {:?}", steps);

}
