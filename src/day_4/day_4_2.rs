use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug, Clone)]
struct Card{
    id: usize,
    winning_numbers: Vec<usize>,
    scratch_numbers: Vec<usize>,
    points: usize,
    amount_of_cards: usize,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_4/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    let mut cards: Vec<Card> = Vec::new();
    let mut cards_map: HashMap<usize, usize> = HashMap::new();
    for line in lines {
        cards.push(process_txt(line));
        cards_map.insert(cards.last().unwrap().id, 1);
    }

    for (i,card) in cards.iter().enumerate(){
        for index in i+1..i+card.points+1{
            debug!("Card: {:?}", card);
            debug!("Index: {}", index);
            debug!("Card points: {}", card.points);
            // cards[index+1].amount_of_cards += 1*card.points;
            debug!("Before Card from cards_map:{:?}", cards_map.get(&(index+1)).unwrap());
            cards_map.insert(index+1, cards_map.get(&(index+1)).unwrap() + 1*cards_map.get(&(i+1)).unwrap());
            debug!("Card from cards_map:{:?}", cards_map.get(&(index+1)).unwrap());
            debug!("Cards_map: {:?}", cards_map);
        }
    }

    info!("Cards map: {:?}", cards_map);
    info!("Cards: {:?}", cards);
    let total: usize = cards.iter().map(|card| card.points).sum();
    let total_map: usize = cards_map.iter().map(|(key, value)| value).sum();
    info!("Total part 1: {}", total);
    info!("Total part 2: {}", total_map);

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
                total += 1;
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
        amount_of_cards: 1,
    }
}