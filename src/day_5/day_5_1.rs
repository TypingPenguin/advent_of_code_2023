use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug)]
struct TypeMap {
    map: Vec<Map>,
    mapping_name: String,
}

#[derive(Debug)]
struct Map {
    source_start: usize,
    source_end: usize,
    destination_start: usize,
    destination_end: usize,
}

static mut paragraph_index: usize = 0;
// static mut seeds: Vec<usize> = Vec::new();

pub(crate) unsafe fn run() {

    let mut seeds: Vec<usize> = Vec::new();

    let input = load_txt::load_txt_file("src/day_5/data_final.txt");




    let mut complete_map: Vec<TypeMap> = Vec::new();
    for i in 0..7 {
        complete_map.push(TypeMap{
            map: Vec::new(),
            mapping_name: String::new(),
        });
    }

    // split the string on newlines
    let lines = input.split("\n");

    for line in lines {
        process_txt(line, &mut complete_map, &mut seeds);
    }


    let mut destination_seed: Vec<Vec<usize>> = Vec::new();

    //Find mapping per seed
    for seed in &seeds{
        let mut destination_seed_vec: Vec<usize> = Vec::new();
        destination_seed_vec.push(*seed);
        for map in &complete_map {
            info!("Map: {:?}", map);
            let mut found_mapping = false;
            for map_map in &map.map {
                if map_map.source_start <= *destination_seed_vec.last().unwrap() && map_map.source_end >= *destination_seed_vec.last().unwrap() {
                    if !found_mapping {

                        let mut destination_number = (map_map.destination_start as isize - map_map.source_start as isize);
                        let mut destination_isize = *destination_seed_vec.last().unwrap() as isize + destination_number;
                        let mut destination = destination_isize as usize;
                        if map_map.destination_start == 18 {
                            info!("Map map: {:?}", map_map);
                            info!("Destination differential: {}", destination_number);
                            info!("Destination isize: {}", destination_isize);
                            info!("Destination: {}", destination);
                            info!("Seed: {}", seed);
                        }
                        destination_seed_vec.push(destination);
                        found_mapping = true;
                    }
                }
            }
            if !found_mapping {
                destination_seed_vec.push(*destination_seed_vec.last().unwrap());
            }
        }
        destination_seed.push(destination_seed_vec);
    }

    info!("Seeds: {:?}", seeds);
    let mut lowest_location = usize::MAX;
    for (i, seed) in destination_seed.iter().enumerate(){
        info!("Mapping: {:?} -> {:?}", seeds[i],seed.last().unwrap());
        if seed.last().unwrap() < &lowest_location {
            lowest_location = *seed.last().unwrap();
        }
    }
    info!("Destination seed: {:?}", destination_seed);
    info!("Lowest location: {:?}", lowest_location);


}
fn process_txt(string: &str, complete_map: &mut Vec<TypeMap>, seeds: &mut Vec<usize>){

    if string.contains("map") {
        unsafe {
            paragraph_index += 1;
        }
    } else if string.contains(":") && string.contains("seeds") {
        unsafe {
            let seed_vector = string.split(":").last().unwrap().trim().replace("\r", "").split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            for seed in seed_vector {
                seeds.push(seed);
            }
        }
    } else {
        let string_clean = string.replace("\r","");
        if string_clean.len() > 0{
            let string_split = string_clean.split(" ").collect::<Vec<&str>>();
            let parse_length = |x: &str| {
                let x_parsed = x.parse::<usize>().unwrap();
                if x_parsed == 0 {
                    0
                } else {
                    x_parsed -1
                }
            };
            let range = parse_length(string_split[2]);
            let mut map = Map{
                source_start: string_split[1].parse::<usize>().unwrap(),
                source_end: string_split[1].parse::<usize>().unwrap()+range,
                destination_start: string_split[0].parse::<usize>().unwrap(),
                destination_end: string_split[0].parse::<usize>().unwrap()+range,
            };
            unsafe {
                complete_map[paragraph_index-1].map.push(map);
            }
        }

    }



}