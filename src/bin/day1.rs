use std::fs::read_to_string;

fn main() {
    let file_data = read_to_string("src/bin/day1.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let mut total: u32 = 0;
    for line in &lines {
        let digits: String = line.chars().filter(|&c| c.is_digit(10)).collect();
        total +=
            (digits.chars().next().unwrap().to_string() +
            &digits.chars().last().unwrap().to_string()).parse::<u32>().unwrap();
    }
    println!("Part 1: {total}");

    /* Part 2 */
    total = 0;
    // check for an a number as a word
    let words = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let word_number_pairs: Vec<_> = numbers.iter().zip(words.iter()).collect();
    for line in &lines {

        // go forward an index at a time
        let first_digit: char = {
            let mut this_char= '_';
            for index in 0..line.len() {
                this_char = line.chars().nth(index).unwrap();
                // check for a numeric character
                if this_char.is_digit(10) {
                    break;
                }

                // check for an a number as a word
                let mut found: bool = false;
                for (number, word) in &word_number_pairs {
                    if line.get(index..line.len()).unwrap().starts_with(**word) {
                        this_char = **number;
                        found = true;
                        break
                    }
                }
                if found {
                    break;
                }
            }
            this_char
        };

        // go backward an index at a time
        let second_digit: char = {
            let mut this_char = '_';
            for index in (0..line.len()).rev() {
                // check for a numeric character
                this_char = line.chars().nth(index).unwrap();
                if this_char.is_digit(10) {
                    break;
                }

                // check for an a number as a word
                let mut found: bool = false;
                for (number, word) in &word_number_pairs {
                    if line.get(index..line.len()).unwrap().starts_with(**word) {
                        this_char = **number;
                        found = true;
                        break
                    }
                }
                if found {
                    break;
                }
            }
            this_char
        };

        total += (first_digit.to_string() + &second_digit.to_string()).parse::<u32>().unwrap();

    }
    println!("Part 2: {total}");
}