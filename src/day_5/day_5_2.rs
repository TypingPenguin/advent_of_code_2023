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
    let mut index = 0;
    'a: loop {
        if index % 100000 == 0 {
            info!("Index: {}", index);
        }
        let mut destination_seed_vec: Vec<usize> = Vec::new();
        destination_seed_vec.push(index);
        for map in complete_map.iter().rev() {
            let mut found_mapping = false;
            for map_map in &map.map {
                if map_map.destination_start <= *destination_seed_vec.last().unwrap() && map_map.destination_end >= *destination_seed_vec.last().unwrap() {
                    if !found_mapping {
                        let mut destination_differential = (map_map.destination_start as isize - map_map.source_start as isize);
                        let mut destination_isize = *destination_seed_vec.last().unwrap() as isize - destination_differential as isize;
                        let mut destination = destination_isize as usize;

                        debug!("Map map: {:?}", map_map);
                        debug!("Destination differential: {}", destination_differential);
                        debug!("Destination isize: {}", destination_isize);
                        debug!("Destination: {}", destination);

                        destination_seed_vec.push(destination);
                        found_mapping = true;
                    }
                }
            }
            if !found_mapping {
                destination_seed_vec.push(*destination_seed_vec.last().unwrap());
            }
        }
        for i in (0..seeds.len()).step_by(2) {
            if seeds[i] <= *destination_seed_vec.last().unwrap() && seeds[i] + seeds[i + 1] >= *destination_seed_vec.last().unwrap() {
                break 'a;
            }
        }
        destination_seed.push(destination_seed_vec);
        index += 1;
    }
    info!("Lowest value is: {:?}", index);


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