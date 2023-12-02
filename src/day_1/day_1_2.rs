use crate::load_txt;
use log::{debug, info};

pub(crate) fn run() {
    let input = load_txt::load_txt_file("src/day_1/data_final.txt");

    // split the string on newlines
    let lines = input.split("\n");

    let mut total: u32 = 0;
    for line in lines {
        total += number_from_line(line.to_string());
    }
    info!("Total: {}", total);
}

static DIGIT_STRINGS: [&str; 9] = [
"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn number_from_line(input: String) -> u32 {
    info!("Input String for function number_from_line {}.", input);
    //go through every element of the string and find the numbers
    let mut numbers_string = String::new();
    let mut position_digits: Vec<isize> = Vec::new();
    for (i,c) in input.char_indices() {
        if c.is_digit(10) {
            numbers_string.push(c);
            position_digits.push(i.try_into().unwrap());
        }
    }

    //go through every element of the string and find the numbers
    let mut numbers_text_string = String::new();
    let mut position_text_digits: Vec<isize> = Vec::new();
    for (i, string) in DIGIT_STRINGS.iter().enumerate() {
        if input.contains(string) {
            find_text_digit(input.clone(), &mut numbers_text_string, &mut position_text_digits, i, string);
        }
    }

    // sort the numbers on order of appearance
    let mut indices: Vec<usize> = (0..position_text_digits.len()).collect();

    indices.sort_by_key(|&i| position_text_digits[i]);

    debug!("Position digits {:?}", position_text_digits);
    debug!("Numbers {:?}", numbers_text_string);
    debug!("Indices {:?}", indices);

    position_text_digits = indices.iter().map(|&i| position_text_digits[i]).collect();
    numbers_text_string = indices.iter().map(|&i| numbers_text_string.chars().nth(i).unwrap()).collect();

    debug!("Position digits after sorting{:?}", position_text_digits);
    debug!("Numbers after sorting {:?}", numbers_text_string);


    debug!("Numbers written out in the string {:?}", numbers_text_string);


    let mut length = numbers_string.len();
    let mut length_text = numbers_text_string.len();
    let mut final_number_string = String::new();

    // If there are no numbers in the string, add the max and min usize values to the vector
    if length == 0 {
        position_digits.push(std::isize::MAX);
        position_digits.push(std::isize::MIN);
        length = 2;
    }
    if length_text == 0 {
        position_text_digits.push(std::isize::MAX);
        position_text_digits.push(std::isize::MIN);
        length_text = 2;
    }

    // Look which has lower index add that one to the string
    if position_digits[0] < position_text_digits[0] {
        final_number_string.push(numbers_string.chars().nth(0).unwrap());
    } else {
        final_number_string.push(numbers_text_string.chars().nth(0).unwrap());
    }


    // Look which has higher index add that one to the string
    if position_digits[length - 1] > position_text_digits[length_text - 1] {
        final_number_string.push(numbers_string.chars().nth(length - 1).unwrap());
    } else {
        final_number_string.push(numbers_text_string.chars().nth(length_text - 1).unwrap());
    }

    debug!("numbers and positions {:?} {:?}", numbers_string, position_digits);
    debug!("numbers and positions text {:?} {:?}", numbers_text_string, position_text_digits);


    debug!("Value of String before transformation (only contains two last digits) {} ", final_number_string);
    let return_value = final_number_string.parse::<u32>().unwrap();
    info!("Value to be returned: {}.", return_value);
    // let return_value = 0;
    return_value
}

fn find_text_digit(mut input: String, numbers_text_string: &mut String, position_text_digits: &mut Vec<isize>, i: usize, string: &&str) {
    debug!("Input String for function find_text_digit {}.", string);
    debug!("Input String for function find_text_digit {}.", input);
    position_text_digits.push(input.find(string).unwrap() as isize);
    let new_i = i + 1;
    numbers_text_string.push_str(&new_i.to_string());
    input = input.replacen(string, &std::iter::repeat('~').take(string.len()).collect::<String>(), 1);
    debug!("Input String for function find_text_digit {}.", input);
    if input.contains(string) {
        find_text_digit(input, numbers_text_string, position_text_digits, i, string);
    }
}