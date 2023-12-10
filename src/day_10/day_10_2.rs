use std::borrow::Borrow;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

static VALUE: usize = 4;

#[derive(Debug)]
struct Engine {
    engine: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum Direction {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Empty,
    Start,
}

impl Direction{
    fn next_direction(&self, origin: MoveDirection) -> MoveDirection {
        match self {
            Direction::NS => {
                match origin {
                    MoveDirection::S => MoveDirection::S,
                    MoveDirection::N => MoveDirection::N,
                    _ => MoveDirection::None,
                }
            },
            Direction::EW => {
                match origin {
                    MoveDirection::E => MoveDirection::E,
                    MoveDirection::W => MoveDirection::W,
                    _ => MoveDirection::None
                }
            },
            Direction::NE =>  {
                match origin {
                    MoveDirection::S => MoveDirection::E,
                    MoveDirection::W => MoveDirection::N,
                    _ => MoveDirection::None
                }
            },
            Direction::NW => {
                match origin {
                    MoveDirection::S => MoveDirection::W,
                    MoveDirection::E => MoveDirection::N,
                    _ => MoveDirection::None
                }
            },
            Direction::SE =>  {
                match origin {
                    MoveDirection::W => MoveDirection::S,
                    MoveDirection::N => MoveDirection::E,
                    _ => MoveDirection::None
                }
            },
            Direction::SW =>  {
                match origin {
                    MoveDirection::E => MoveDirection::S,
                    MoveDirection::N => MoveDirection::W,
                    _ => MoveDirection::None
                }
            },
            Direction::Empty => MoveDirection::None,
            Direction::Start => MoveDirection::None,
        }
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum MoveDirection {
    N,
    S,
    E,
    W,
    None,
}

impl MoveDirection {
    fn coordinates(&self) -> [i32; 2] {
        match self {
            MoveDirection::N => [-1, 0],
            MoveDirection::S => [1, 0],
            MoveDirection::E => [0, 1],
            MoveDirection::W => [0, -1],
            MoveDirection::None => [0, 0],
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_direction() {
        let direction = Direction::NS;
        let origin = MoveDirection::N;
        let new_direction = direction.next_direction(origin);
        assert_eq!(new_direction, MoveDirection::S);
    }

    #[test]
    fn test_direction_2() {
        let mut map = Vec::new();
        let line = vec![0,1,2];
        let line2 = vec![3,4,5];
        let line3 = vec![6,7,8];
        map.push(line);
        map.push(line2);
        map.push(line3);
        let position = [1,1];
        let direction = Direction::NS;
        let origin = MoveDirection::N;
        let new_direction = direction.next_direction(origin);
        let new_position  = [(position[0] + new_direction.coordinates()[0]) as usize, (position[1] + new_direction.coordinates()[1]) as usize];
        assert_eq!(new_position, [2,1]);
        assert_eq!(map[new_position[0]][new_position[1]], 7);

    }
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_10/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    let mut map: Vec<Vec<Direction>> = Vec::new();
    let mut start: [usize;2] = [0, 0];

    // Parse the input to create a map with directions
    for (i,line) in lines.into_iter().enumerate(){
        let mut line_vec: Vec<Direction> = Vec::new();
        for (j,char) in line.trim().chars().enumerate(){
            match char {
                '.' => line_vec.push(Direction::Empty),
                '|' => line_vec.push(Direction::NS),
                '-' => line_vec.push(Direction::EW),
                'L' => line_vec.push(Direction::NE),
                'J' => line_vec.push(Direction::NW),
                '7' => line_vec.push(Direction::SW),
                'F' => line_vec.push(Direction::SE),
                'S' => {
                    start = [i,j];
                    line_vec.push(Direction::Start)
                },
                _ => panic!("Unknown character: {:?}", char),
            }
        }
        map.push(line_vec);
    }

    debug!("Map: {:?}", map);
    debug!("Start: {:?}", start);
    debug!("Map start: {:?}", map[start[0]][start[1]]);

    let directions = vec![MoveDirection::E, MoveDirection::W, MoveDirection::N, MoveDirection::S];


    let mut big_vector_map: Vec<Vec<Vec<usize>>> = Vec::new();
    // go in all 4 directions from start point
    for direction in directions {
        let mut found_loop = false;
        let mut current_position = start;
        let mut current_direction = direction;
        let mut index = 0;
        let mut map_direction: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];
        loop{
            let next_move = current_direction.coordinates();
            // check not out of bounds
            if current_position[0] as i32 + next_move[0] < 0 || current_position[0] as i32 + next_move[0] > map.len() as i32 {
                break;
            }
            if current_position[1] as i32 + next_move[1] < 0 || current_position[1] as i32 + next_move[1] > map[0].len() as i32 {
                break;
            }
            debug!("Current position: {:?}", current_position);
            debug!("Next move: {:?}", next_move);
            current_position = [(current_position[0] as i32 + next_move[0]) as usize, (current_position[1] as i32 + next_move[1]) as usize];
            debug!("New position: {:?}", current_position);
            let new_pipe =  map[current_position[0]][current_position[1]].clone();
            debug!("New pipe: {:?}", new_pipe);
            debug!("Current direction: {:?}", current_direction);
            current_direction = new_pipe.next_direction(current_direction);
            debug!("New direction: {:?}", current_direction);

            if current_position == start {
                found_loop = true;
                break;
            }

            // check if the pipe was available
            if current_direction == MoveDirection::None {
                break;
            }

            // check if we are at the start again


            // count steps
            index += 1;
            map_direction[current_position[0]][current_position[1]] = index;

        }
        pretty_print_2d(&map_direction);
        debug!("Map direction: {:?}", map_direction);
        if found_loop{
            big_vector_map.push(map_direction);
        }
    }

    // overlap each vector map in big_vector_map and keep lowest numbers in each cell
    let mut final_map: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];
    for map in big_vector_map {
        pretty_print_2d(&map);
        for (i,line) in map.iter().enumerate() {
            for (j,cell) in line.iter().enumerate() {
                if final_map[i][j] == 0 {
                    final_map[i][j] = *cell;
                } else if *cell != 0 && *cell < final_map[i][j] {
                    final_map[i][j] = *cell;
                }
            }
        }
    }
    pretty_print_2d(&final_map);

    // find the highest number in the map
    let mut highest_number = 0;
    for line in &final_map {
        for cell in line {
            if cell > &highest_number {
                highest_number = *cell;
            }
        }
    }

    info!("Highest number: {:?}", highest_number);

    let mut map_loop_wall: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];
    for (i,line) in final_map.iter().enumerate() {
        for (j,cell) in line.iter().enumerate() {
            if *cell != 0 {
                if map[i][j] == Direction::EW {
                    map_loop_wall[i][j] = 1;
                } else if map[i][j] == Direction::NW || map[i][j] == Direction::NE {
                    map_loop_wall[i][j] = 3;
                } else if map[i][j] == Direction::SW || map[i][j] == Direction::SE {
                    map_loop_wall[i][j] = 4;
                } else {
                    map_loop_wall[i][j] = 2;
                }

            }
        }
    }
    map_loop_wall[start[0]][start[1]] = VALUE;
    pretty_print_2d(&map_loop_wall);


    let mut inside_bool = false;
    final_map[start[0]][start[1]] = 1;
    let mut counter = 0;

    let mut new_map: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];
    let mut still_in_wall = false;


    let mut counter_up = 0;
    let mut counter_down = 0;
    for (i, line) in map_loop_wall.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            // change when cell is not 0
            // if wall
            if cell > 1 {
                if cell == 2 {
                    counter_down += 1;
                    counter_up += 1;
                } else if cell == 3 {
                    counter_up += 1;
                } else if cell == 4 {
                    counter_down += 1;
                }
            } else if cell == 0 {
                if (counter_up % 2 != 0 && counter_down % 2 != 0) {
                    counter += 1;
                    new_map[i][j] = 1;
                }
            }
        }
    }
    pretty_print_2d(&new_map);
    info!("Counter: {:?}", counter);

    info!("Above: {:?}", map[start[0] - 1][start[1]]);
    info!("Below: {:?}", map[start[0] + 1][start[1]]);
    info!("Left: {:?}", map[start[0]][start[1] - 1]);
    info!("Right: {:?}", map[start[0]][start[1] + 1]);


}
fn pretty_print_2d(map: &Vec<Vec<usize>>) {
    println!("Pretty print:");
    for line in map {
        for char in line {
            print!("{:1} ", char);
        }
        println!();
    }
    println!("Pretty print end");
}