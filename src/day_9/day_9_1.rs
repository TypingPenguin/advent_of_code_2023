use std::borrow::Borrow;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug)]
struct History {
    history: Vec<Vec<isize>>,
    predicted: isize,
}
pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_9/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    let mut histories: Vec<History> = Vec::new();

    // for each line, split on spaces and parse to isize
    // Create a history struct for each line
    for line in lines{
        let vec: Vec<isize> = line.split(" ").map(|s| s.trim().parse::<isize>().unwrap()).collect();
        let history = History{
            history: vec![vec],
            predicted: 0,
        };
        histories.push(history);
    }

    //Find all the histories difference until all 0s
    // Also adding a 0 to the end of the history
    for history in histories.iter_mut(){
        let original_history = history.history[0].clone();
        let mut current_history = history.history[0].clone();
        loop{
            let mut new_history:Vec<isize> = Vec::new();
            for i in 0..current_history.len()-1{
                let difference = current_history[i+1] - current_history[i];
                new_history.push(difference);
            }
            current_history = new_history.clone();
            //check if new_history only contains 0s
            let mut all_zeros = true;
            for i in 0..new_history.len(){
                if new_history[i] != 0{
                    all_zeros = false;
                }
            }
            if all_zeros{
                new_history.push(0);
                history.history.push(new_history.clone());
                break;
            }
            history.history.push(new_history.clone());
        }
    }



    // Find the predicted number
    for history in histories.iter_mut(){
        // go through the history backwards
        for i in (0..history.history.len()-1).rev(){
            let mut predicted = 0;
            predicted += history.history[i].last().unwrap() + history.history[i+1].last().unwrap();

            debug!("Current history: {:?}", history.history[i]);
            debug!("Last element of this history: {:?}", history.history[i].last().unwrap());
            debug!("History below to add: {:?}", history.history[i+1]);
            debug!("Last element of history below: {:?}", history.history[i+1].last().unwrap());
            debug!("Predicted number: {:?}", predicted);


            history.history[i].push(predicted);

            if i == 0{
                history.predicted = predicted;
            }
        }

    }
    debug!("{:?}", histories);

    // sum all predicted values of the histories
    let mut sum = 0;
    for history in histories.iter(){
        sum += history.predicted;
    }
    info!("Sum of all predicted values: {:?}", sum);

}
