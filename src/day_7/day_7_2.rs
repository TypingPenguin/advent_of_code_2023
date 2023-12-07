use std::borrow::Borrow;
use std::io::BufRead;
use std::str::Split;
use env_logger::init_from_env;
use crate::load_txt;
use log::{debug, info};

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    cards_usize: Vec<usize>,
    bid: usize,
}

#[derive(Debug, Clone)]
struct Type_Hand {
    hands: Vec<Hand>,
}

#[derive(Debug, Clone)]
struct Complete_Hand {
    complete_hand: Vec<Type_Hand>,
}

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_7/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");
    let amount_of_lines = lines.clone().count();

    let mut vector = get_object_from_lines(&lines);

    let mut j_max = 0;
    debug!("Vector: {:?}", vector);

    let mut complete_hand = Complete_Hand {
        complete_hand: Vec::new(),
    };

    // create 7 types of type hands
    for i in 0..7 {
        complete_hand.complete_hand.push(Type_Hand {
            hands: Vec::new(),
        });
    }

    for hand in vector {
        let mut chars_seen: Vec<char> = Vec::new();
        let mut same_cards: Vec<usize> = Vec::new();
        let mut j_counter = 0;
        for (i, char) in hand.cards.chars().into_iter().enumerate() {
            let mut same_card = 0;
            if chars_seen.contains(&char) {
                continue;
            }
            for j in i..hand.cards.len() {
                if char == 'J' && char == hand.cards.chars().nth(j).unwrap(){
                    j_counter += 1;
                    chars_seen.push(char);
                } else if char == hand.cards.chars().nth(j).unwrap() {
                    same_card += 1;
                    chars_seen.push(char);
                }
            }
            same_cards.push(same_card);
        }
        j_max = j_max.max(j_counter);

        debug!("Hand: {:?}", hand);
        debug!("Same cards: {:?}", same_cards);
        //     Sort where to put in complete hand

        //Five of a kind
        if same_cards.contains(&5){
            match j_counter{
                5 => panic!("Impossible"),
                4 => panic!("Impossible"),
                3 => panic!("Impossible"),
                2 => panic!("Impossible"),
                1 => panic!("Impossible"),
                _ => complete_hand.complete_hand[0].hands.push(hand),
            }

        // Four of a kind
        } else if same_cards.contains(&4){
            match j_counter{
                5 => panic!("Impossible"),
                4 => panic!("Impossible"),
                3 => panic!("Impossible"),
                2 => panic!("Impossible"),
                1 => complete_hand.complete_hand[0].hands.push(hand),
                _ => complete_hand.complete_hand[1].hands.push(hand),
            }

        // Full house
        } else if same_cards.contains(&3) && same_cards.contains(&2){
            match j_counter{
                5 => panic!("Impossible"),
                4 => panic!("Impossible"),
                3 => panic!("Impossible"),
                2 => panic!("Impossible"),
                1 => panic!("Impossible"),
                _ => complete_hand.complete_hand[2].hands.push(hand),
            }

        // Three of a kind
        } else if same_cards.contains(&3){
            match j_counter{
                5 => panic!("Impossible"),
                4 => panic!("Impossible"),
                3 => panic!("Impossible"),
                2 => complete_hand.complete_hand[0].hands.push(hand),
                1 => complete_hand.complete_hand[1].hands.push(hand),
                _ => complete_hand.complete_hand[3].hands.push(hand),
            }

        // Two pairs
        } else if same_cards.iter().filter(|&n| *n == 2).count() == 2 {
            match j_counter{
                5 => panic!("Impossible"),
                4 => panic!("Impossible"),
                3 => panic!("Impossible"),
                2 => panic!("Impossible"),
                1 => complete_hand.complete_hand[2].hands.push(hand),
                _ => complete_hand.complete_hand[4].hands.push(hand),
            }

        // One pair
        } else if same_cards.contains(&2){
            match j_counter{
                5 => panic!("Impossible"),
                4 => panic!("Impossible"),
                3 => complete_hand.complete_hand[0].hands.push(hand),
                2 => complete_hand.complete_hand[1].hands.push(hand),
                1 => complete_hand.complete_hand[3].hands.push(hand),
                _ => complete_hand.complete_hand[5].hands.push(hand),
            }

        // High card
        } else {
            match j_counter{
                5 => complete_hand.complete_hand[0].hands.push(hand),
                4 => complete_hand.complete_hand[0].hands.push(hand),
                3 => complete_hand.complete_hand[1].hands.push(hand),
                2 => complete_hand.complete_hand[3].hands.push(hand),
                1 => complete_hand.complete_hand[5].hands.push(hand),
                _ => complete_hand.complete_hand[6].hands.push(hand),
            }
        }



    }

    for hand in &mut complete_hand.complete_hand {
        debug!("Hand: {:?}", hand);
    }

    debug!("Complete hand: {:?}", complete_hand);
    // sort the complete hand on highest first card
    for i in 0..complete_hand.complete_hand.len() {
        for j in 0..complete_hand.complete_hand[i].hands.len() {
            // sort the hands on highest first card with the other hands
            for k in 0..complete_hand.complete_hand[i].hands.len() {
                let first_hand = complete_hand.complete_hand[i].hands[j].cards_usize.clone();
                let second_hand = complete_hand.complete_hand[i].hands[k].cards_usize.clone();
                if (first_hand[0] > second_hand[0]) || (first_hand[0] == second_hand[0] && first_hand[1] > second_hand[1]) || (first_hand[0] == second_hand[0] && first_hand[1] == second_hand[1] && first_hand[2] > second_hand[2]) || (first_hand[0] == second_hand[0] && first_hand[1] == second_hand[1] && first_hand[2] == second_hand[2] && first_hand[3] > second_hand[3]) || (first_hand[0] == second_hand[0] && first_hand[1] == second_hand[1] && first_hand[2] == second_hand[2] && first_hand[3] == second_hand[3] && first_hand[4] > second_hand[4]) {
                    let temp = complete_hand.complete_hand[i].hands[j].clone();
                    complete_hand.complete_hand[i].hands[j] = complete_hand.complete_hand[i].hands[k].clone();
                    complete_hand.complete_hand[i].hands[k] = temp;
                }
            }
        }
    }

    let mut rank = amount_of_lines;
    let mut total = 0;
    for i in 0..complete_hand.complete_hand.len() {
        for j in 0..complete_hand.complete_hand[i].hands.len() {
            total += complete_hand.complete_hand[i].hands[j].bid * rank;
            rank -= 1;
        }
    }
    debug!("Complete hand: {:?}", complete_hand);
    debug!("Total: {:?}", total);
    debug!("Rank: {:?}", rank);
    debug!("J max: {:?}", j_max);
}

fn get_object_from_lines(lines: &Split<&str>) -> Vec<Hand> {
    let mut vector: Vec<Hand> = Vec::new();
    //use regex parser to get the data
    for line in lines.clone() {
        let mut parts = line.split(" ");
        let cards = parts.next().unwrap().trim();
        let bid = parts.next().unwrap().trim().parse::<usize>().unwrap();

        // cards_usize but handle that T = 10 J = 11 Q = 12 K = 13 A = 14
        let mut cards_usize: Vec<usize> = Vec::new();
        for card in cards.chars() {
            match card {
                'T' => cards_usize.push(10),
                'J' => cards_usize.push(0),
                'Q' => cards_usize.push(12),
                'K' => cards_usize.push(13),
                'A' => cards_usize.push(14),
                _ => cards_usize.push(card.to_string().parse::<usize>().unwrap()),
            }
        }

        let hand = Hand {
            cards: cards.to_string(),
            cards_usize: cards_usize,
            bid: bid,
        };
        vector.push(hand);
    }
    vector
}
