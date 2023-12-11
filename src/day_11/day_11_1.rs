use std::borrow::Borrow;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};
use num_integer::Roots;

#[derive(Debug)]
struct Engine {
    engine: Vec<Vec<char>>,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_11/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");


    let mut galaxy_boolean_map: Vec<Vec<bool>> = Vec::new();



    for (i,line) in lines.enumerate() {
        let mut galaxy_row: Vec<bool> = Vec::new();

        for (j,c) in line.trim().chars().enumerate() {
            if c == '#' {
                galaxy_row.push(true);
            } else {
                galaxy_row.push(false);
            }
        }
        galaxy_boolean_map.push(galaxy_row);
    }

    let mut max_row = galaxy_boolean_map.len();
    let mut row = 0;

    while row < max_row{
        let mut empty_line = true;
        for j in 0..galaxy_boolean_map[row].len() {
            if galaxy_boolean_map[row][j] == true {
                empty_line = false;
            }
        }
        if empty_line == true {
            galaxy_boolean_map.insert(row, vec![false; galaxy_boolean_map[row].len()]);
            row += 1;
            max_row += 1;
        }
        row += 1;
    }

    debug!("Printing the galaxy with added rows");
    print_galaxy(&mut galaxy_boolean_map);

    //find empty columns, if empty, add another column
    let mut max_col = galaxy_boolean_map[0].len();
    let mut col = 0;

    while col < max_col {
        let mut empty_col = true;
        for i in 0..galaxy_boolean_map.len() {
            if galaxy_boolean_map[i][col] == true {
                empty_col = false;
            }
        }
        if empty_col == true {
            for i in 0..galaxy_boolean_map.len() {
                galaxy_boolean_map[i].insert(col, false);
            }
            col += 1;
            max_col += 1;
        }
        col += 1;

    }

    debug!("Printing the galaxy with added columns");
    print_galaxy(&mut galaxy_boolean_map);

    let mut galaxy_id = 0;
    let mut galaxies: Vec<Vec<usize>> = Vec::new();

    for i in 0..galaxy_boolean_map.len() {
        for j in 0..galaxy_boolean_map[i].len() {
            if galaxy_boolean_map[i][j] == true {
                galaxies.push(vec![galaxy_id, i, j]);
                galaxy_id += 1;
            }
        }
    }

    debug!("Printing location of galaxies, vector is [galaxy_id, row, col]");
    for galaxy in galaxies.iter() {
        println!("{:?}", galaxy);
    }

    let mut galaxy_distances: Vec<Vec<usize>> = Vec::new();
    // for each pair of galaxies, find the distance between them
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i+1..galaxies.len() {
            let distance = find_distance(galaxy, &galaxies[j]);
            println!("Distance between {:?} and {:?} is {}", galaxy[0], galaxies[j][0], distance);
            galaxy_distances.push(vec![galaxy[0], galaxies[j][0], distance]);
        }
    }
    //sum all the distances in galaxy_distances
    let sum = galaxy_distances.iter().filter_map(|x| x.get(2)).sum::<usize>();
    info!("The sum of all distances is {}", sum);


}

// find the distance between two galaxies
fn find_distance(galaxy_1: &Vec<usize>, galaxy_2: &Vec<usize>) -> usize {
    let row_1 = galaxy_1[1] as f64;
    let col_1 = galaxy_1[2] as f64;
    let row_2 = galaxy_2[1] as f64;
    let col_2 = galaxy_2[2] as f64;

    debug!("galaxy_1: {:?}", galaxy_1);
    debug!("galaxy_2: {:?}", galaxy_2);
    debug!("row_1: {}, col_1: {}, row_2: {}, col_2: {}", row_1, col_1, row_2, col_2);


    let distance = ((row_1 - row_2).abs() + (col_1 - col_2).abs());
    debug!("distance: {}", distance);

    return distance as usize;
}

fn print_galaxy(galaxy_location: &mut Vec<Vec<bool>>) {
//print the galaxy
    for galaxy_row in galaxy_location.iter() {
        for galaxy_col in galaxy_row.iter() {
            if galaxy_col == &true {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
