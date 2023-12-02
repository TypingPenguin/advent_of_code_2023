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

fn number_from_line(input: String) -> u32 {
    debug!("Input for function {}.", input);
    //go through every element of the string and find the numbers
    let mut numbers_string = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            numbers_string.push(c);
        }
    }
    let length = numbers_string.len();
    // let mut final_number = String::new();
    // final_number.push(char::try_from(numbers[0]).unwrap());
    // final_number.push(char::try_from(numbers[length - 1]).unwrap());
    let mut final_number_string = String::new();
    final_number_string.push(numbers_string.chars().nth(0).unwrap());
    final_number_string.push(numbers_string.chars().nth(length - 1).unwrap());


    debug!("Value of String before transformation (only contains two last digits) {} ", final_number_string);
    let return_value = final_number_string.parse::<u32>().unwrap();
    debug!("Value to be returned: {}.", return_value);
    // let return_value = 0;
    return_value
}