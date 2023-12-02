use std::borrow::Borrow;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    pulls: Vec<Pull>,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_2/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        games.push(get_object(line));
    }



    let mut total: u32 = 0;
    'game: for game in games {

        let mut min_pull = Pull {
            red: 0,
            green: 0,
            blue: 0,
        };

        for pull in game.pulls {
            if pull.red > min_pull.red {
                min_pull.red = pull.red;
            }
            if pull.green > min_pull.green {
                min_pull.green = pull.green;
            }
            if pull.blue > min_pull.blue {
                min_pull.blue = pull.blue;
            }
        }
        total += min_pull.blue * min_pull.green * min_pull.red;
    }
    info!("Total: {}", total);


}

// Returns true if the pull is valid
fn check_pull(pull: &Pull) -> bool {
    if too_much_red(&pull) {
        return false;
    }
    if too_much_green(&pull) {
        return false;
    }
    if too_much_blue(&pull) {
        return false;
    }
    return true;
}

fn too_much_red(pull: &Pull) -> bool {
    return pull.red > 12
}
fn too_much_green(pull: &Pull) -> bool {
    return pull.green > 13
}
fn too_much_blue(pull: &Pull) -> bool {
    return pull.blue > 14
}

fn get_object(input: &str) -> Game {
    let mut strings = input.split(":");
    let game_string = "Game ";
    let game_id = strings.next().unwrap().to_string().replace(game_string, "");
    let binding = strings.next().unwrap().to_string();
    let pulls = binding.split(";");
    let mut pulls_vec: Vec<Pull> = Vec::new();

    for pull in pulls {
        let mut pull_object = Pull {
            red: 0,
            green: 0,
            blue: 0,
        };
        let pull_string = pull.split(",");
        let mut pull_vec: Vec<u32> = Vec::new();
        for pull_orig in pull_string {
            let mut pull = &pull_orig[1..];
            let mut number_color = pull.split(" ");
            let number = number_color.next().unwrap().parse::<u32>().unwrap();
            let color = number_color.next().unwrap().to_string();
            info!("Color: {:?}", number_color);
            if color.contains("red") {
                pull_object.red = number;
            } else if color.contains("green") {
                pull_object.green = number;
                debug!("Number green: {}", number);
            } else if color.contains("blue") {
                pull_object.blue = number;
            }
        }
        pulls_vec.push(pull_object);
    }
    Game {
        game_id: game_id.parse::<u32>().unwrap(),
        pulls: pulls_vec,
    }
}