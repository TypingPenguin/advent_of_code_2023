use std::borrow::Borrow;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug)]
struct Card{
    id: usize,
    winning_numbers: Vec<usize>,
    scratch_numbers: Vec<usize>,
    points: usize,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_4/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        cards.push(process_txt(line));
    }
    info!("Cards: {:?}", cards);


    let total: usize = cards.iter().map(|card| card.points).sum();
    info!("Total: {}", total);

}

fn process_txt(string: &str) -> Card{
    let mut split_on_semicolon = string.split(":");
    let card_id = split_on_semicolon.next().unwrap().replace("Card", "").chars().filter(|c| *c != ' ').collect::<String>().parse::<usize>().unwrap();
    let mut split_on_bar = split_on_semicolon.next().unwrap().split("|");
    let winning_numbers = split_on_bar.next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|x|
        x.replace(" ","").parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let scratch_numbers = split_on_bar.next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|x|
        x.replace(" ","").replace("\r","").parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut total = 0;
    scratch_numbers.iter().for_each(|x| {
        winning_numbers.iter().for_each(|y| {
            if x == y {
                if total == 0 {
                    total = 1;
                } else {
                    total *= 2;
                }
            }
        })
    });

    debug!("Total: {}", total);
    debug!("Card id: {}", card_id);
    debug!("Winning numbers: {:?}", winning_numbers);
    debug!("Scratch numbers: {:?}", scratch_numbers);
    return Card{
        id: card_id,
        winning_numbers: winning_numbers,
        scratch_numbers: scratch_numbers,
        points: total,
    }
}