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
    let mut engine = Engine {
        engine: Vec::new(),
    };
    for line in lines {
        engine.engine.push(process_txt(line));
    }
    info!("Engine: {:?}", engine);

    let total = find_numbers(engine);


}

static LOOK_AROUND: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0), (1, 0),
    (-1, 1), (0, 1), (1, 1)
];
static LOOK_AROUND_DIGITS: [(i32, i32); 2] = [
    (0, -1), (0, -2)
];

fn find_numbers(engine: Engine) {

    let mut huge_map: Vec<BigMap> = Vec::new();

    let mut engine_signs = Vec::new();

    for (i,line) in engine.engine.iter().enumerate() {
        let mut line_signs = Vec::new();
        for (j,c) in line.iter().enumerate() {
            if !c.is_digit(10) && *c == '*' {
                line_signs.push(1);
            } else {
                line_signs.push(0);
            }
        }
        engine_signs.push(line_signs);
    }

    let mut number_string: Vec<String> = Vec::new();

    let mut map: Vec<Vec<bool>> = vec![vec![false; engine_signs[0].len()]; engine_signs.len()];
    let mut map_two_engines_around: Vec<Vec<bool>> = vec![vec![false; engine_signs[0].len()]; engine_signs.len()];


    // for (mut line,line_value) in engine_signs.iter().enumerate() {
    //     for (column, c) in line_value.iter().enumerate() {
    //         if *c == 1 {
    //             for (x, y) in LOOK_AROUND.iter() {
    //                 number_string.clear();
    //                 let x_i32 = x + line as i32;
    //                 let y_i32 = y + column as i32;
    //                 if x_i32 < 0 || y_i32 < 0 {
    //                     continue;
    //                 }
    //                 if x_i32 >= (engine_signs.len() as i32) || y_i32 >= (engine_signs[0].len() as i32) {
    //                     continue;
    //                 }
    //
    //                 let x = x_i32 as usize;
    //                 let y = y_i32 as usize;
    //                 if engine.engine[x][y].is_digit(10) {
    //                     map_two_engines_around[x][y] = true;
    //                 }
    //             }
    //         }
    //     }
    // }


    for (mut line,line_value) in engine_signs.iter().enumerate() {
        for (column, c) in line_value.iter().enumerate() {
            // if character map has a 1, go check around it
            if *c == 1 {
                for (x, y) in LOOK_AROUND.iter() {
                    number_string.clear();
                    let x_i32 = x + line as i32;
                    let y_i32 = y + column as i32;
                    if x_i32 < 0 || y_i32 < 0 {
                        continue;
                    }
                    if x_i32 >= (engine_signs.len() as i32) || y_i32 >= (engine_signs[0].len() as i32) {
                        continue;
                    }

                    let x = x_i32 as usize;
                    let y = y_i32 as usize;



                    // if the one we found is a digit, look around it
                    if engine.engine[x][y].is_digit(10) {

                        map[x][y] = true;

                        let mut new_y = y;
                        // look to the left of the digit
                        while true {
                            if new_y > 0 {
                                new_y = new_y - 1;
                            } else {
                                break;
                            }

                            if engine.engine[x][new_y].is_digit(10) {
                                map[x][new_y] = true;
                                continue;
                            } else {
                                break;
                            }
                        }
                        let mut new_y = y;
                        // look to the right of the digit
                        while true {

                            new_y = new_y + 1;

                            // TODO: Check if this is correct
                            if  new_y >= engine_signs[0].len() {
                                break;
                            }

                            if engine.engine[x][new_y].is_digit(10) {
                                map[x][new_y] = true;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pretty_print_2d_vector(&mut map);


    //Go through the map and original engine and find the numbers
    let mut number_string: Vec<String> = Vec::new();
    let mut string = String::new();
    let mut engine_location: Vec<Vec<usize>> = Vec::new();


    for (i,line) in engine.engine.iter().enumerate() {
        for (j,c) in line.iter().enumerate() {
            if map[i][j] {
                string.push(*c);

                // find location of the gear
                for (x, y) in LOOK_AROUND.iter() {
                    number_string.clear();
                    let x_i32 = x + i as i32;
                    let y_i32 = y + j as i32;
                    if x_i32 < 0 || y_i32 < 0 {
                        continue;
                    }
                    if x_i32 >= (engine_signs.len() as i32) || y_i32 >= (engine_signs[0].len() as i32) {
                        continue;
                    }

                    let x = x_i32 as usize;
                    let y = y_i32 as usize;
                    if engine_signs[x][y] == 1 {
                        let mut add = true;
                        for engine in &engine_location {
                            if (engine[0] == x && engine[1] == y) {
                                add = false;
                            }
                        }
                        if add {
                            engine_location.push(vec![x,y]);
                        }

                    }
                }

            } else {
                if string.len() > 0 {
                    let mut big_map = BigMap {
                        gearLocations: engine_location,
                        string: string,
                    };
                    huge_map.push(big_map);
                    engine_location = Vec::new();
                    string = String::new();
                }
            }
        }
    }



    info!("Huge map: {:?}", huge_map);

    let mut total = 0;
    for (i,part) in huge_map.iter().enumerate() {
        for gear in &part.gearLocations {
            let mut counter = 0;
            let mut gears = Vec::new();
            gears.push(gear);
            let mut location = i;
            for iter in i+1..huge_map.len(){
                for gear_compare in &huge_map[iter].gearLocations{
                    if gear[0] == gear_compare[0] && gear[1] == gear_compare[1] {
                        counter += 1;
                        location = iter;
                        gears.push(gear_compare);
                    }
                }
            }
            if counter == 1 {
                info!("Found a part: {:?}", part.string.parse::<i32>().unwrap());
                info!("Found second part: {:?}", huge_map[location].string.parse::<i32>().unwrap());

                total += (part.string.parse::<i32>().unwrap() * huge_map[location].string.parse::<i32>().unwrap());
            }
        }


    }

    info!("Total: {}", total);

}

#[derive(Debug)]
struct BigMap {
    gearLocations: Vec<Vec<usize>>,
    string: String,
}


fn pretty_print_2d_vector(map: &mut Vec<Vec<bool>>) {
// Pretty print the map
    for row in map {
        for cell in row {
            if *cell {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!(); // Move to the next line for the next row
    }
}

fn process_txt(line: &str) -> Vec<char> {
    let mut return_vec = Vec::new();
    for c in line.chars() {
        if c != '\r' {
            return_vec.push(c);
        }
    }
    return_vec
}