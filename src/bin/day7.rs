use std::fs::read_to_string;
use std::cmp::{Ordering};
use counter::Counter;
use crate::Hands::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(PartialEq, PartialOrd)]
enum Hands {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

fn card_score(c: char) -> i32 {
    match c {
        'A' => -1,
        'K' => -2,
        'Q' => -3,
        'J' => -4,
        'T' => -5,
        '9' => -6,
        '8' => -7,
        '7' => -8,
        '6' => -9,
        '5' => -10,
        '4' => -11,
        '3' => -12,
        '2' => -13,
        _ => panic!("Unexpected card!")
    }
}
fn hand_score(x: &str) -> Hands {
    let char_counts = x.chars().collect::<Counter<_>>().most_common_ordered();
    let first_count = char_counts[0].1;
    let second_count = if char_counts.len() == 1 {0} else {char_counts[1].1};
    return
        if first_count == 5 {
            FiveOfAKind
        } else if first_count == 4 {
            FourOfAKind
        } else if (first_count == 3) && (second_count == 2) {
            FullHouse
        } else if first_count == 3 {
            ThreeOfAKind
        } else if (first_count == 2) && (second_count == 2) {
            TwoPair
        } else if first_count == 2 {
            OnePair
        } else if first_count == 1 {
            HighCard
        } else {
            panic!("Oh no!")
        };
}
fn cmp(a: &str, b: &str) -> Ordering {
    let (a_score, b_score) = (hand_score(a), hand_score(b));
    return
        if a_score > b_score {
            Ordering::Greater
        } else if a_score < b_score {
            return Ordering::Less
        } else if a_score == b_score {
            for (c1, c2) in a.chars().zip(b.chars()) {
                if card_score(c1) > card_score(c2) {
                    return Ordering::Greater
                } else if card_score(c1) < card_score(c2) {
                    return Ordering::Less
                }
            }
            panic!("Oh no :(")
        } else {
            panic!("Oh no!")
        };
}

fn part_2_card_score(c: char) -> i32 {
    match c {
        'A' => -1,
        'K' => -2,
        'Q' => -3,
        'J' => -14, // lower than in part 1
        'T' => -5,
        '9' => -6,
        '8' => -7,
        '7' => -8,
        '6' => -9,
        '5' => -10,
        '4' => -11,
        '3' => -12,
        '2' => -13,
        _ => panic!("Unexpected card!")
    }
}
fn part_2_hand_score(x: &str) -> Hands {
    let char_counts: Vec<(char, usize)> = x.chars().collect::<Counter<_>>().most_common_ordered();
    let first_count = char_counts[0].1;
    let second_count = if char_counts.len() == 1 {0} else {char_counts[1].1};

    let mut num_jacks = 0usize;
    for (c, count) in &char_counts {
        if *c == 'J' {
            num_jacks = *count;
            break;
        }
    };

    return
        if num_jacks >= 4 { FiveOfAKind }
        else if num_jacks == 3 {
            if second_count == 1 { FourOfAKind } // three of a kind -> four of a kind
            else if second_count == 2 { FiveOfAKind } // full house -> five of a kind
            else { panic!("Unexpected deck {num_jacks}!") }
        } else if num_jacks == 2 {
            if first_count == 3 { FiveOfAKind } // full house -> five of a kind
            else if second_count == 2 { FourOfAKind } // two pair -> four pair
            else if second_count == 1 { ThreeOfAKind } // pair -> three of a kind
            else { panic!("Unexpected deck {num_jacks}!") }
        } else if num_jacks == 1 {
            if first_count == 4 { FiveOfAKind } // four of a kind -> five of a kind
            else if first_count == 3 { FourOfAKind } // three of a kind -> four of a kind
            else if (first_count == 2) && (second_count == 2) { FullHouse } // two pair -> full house
            else if (first_count == 2) && (second_count == 1) { ThreeOfAKind } // one pair -> three of a kind
            else if first_count == 1 { OnePair } // high card -> one pair
            else { panic!("Unexpected deck {num_jacks}!") }
        } else if num_jacks == 0 {
            if first_count == 5 {
                FiveOfAKind
            } else if first_count == 4 {
                FourOfAKind
            } else if (first_count == 3) && (second_count == 2) {
                FullHouse
            } else if first_count == 3 {
                ThreeOfAKind
            } else if (first_count == 2) && (second_count == 2) {
                TwoPair
            } else if first_count == 2 {
                OnePair
            } else if first_count == 1 {
                HighCard
            } else {
                panic!("Oh no!")
            }
        } else {
            panic!("Unknown number of jacks!");
        }
}
fn part_2_cmp(a: &str, b: &str) -> Ordering {
    let (a_score, b_score) = (part_2_hand_score(a), part_2_hand_score(b));
    return
        if a_score > b_score {
            Ordering::Greater
        } else if a_score < b_score {
            return Ordering::Less
        } else if a_score == b_score {
            for (c1, c2) in a.chars().zip(b.chars()) {
                if part_2_card_score(c1) > part_2_card_score(c2) {
                    return Ordering::Greater
                } else if part_2_card_score(c1) < part_2_card_score(c2) {
                    return Ordering::Less
                }
            }
            panic!("Oh no :(")
        } else {
            panic!("Oh no!")
        };
}

fn main() {
    let file_data = read_to_string("src/bin/inputs/day7.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    // read the hands in
    let mut hands_and_bets: Vec<(&str, i64)> = vec![];
    for l in lines {
        let split_line: Vec<&str> = l.split(' ').collect();
        hands_and_bets.push((split_line[0], split_line[1].parse::<i64>().unwrap()));
    }

    hands_and_bets.sort_by(|a, b| cmp(a.0, b.0));
    let result: i64 = hands_and_bets.iter().enumerate().map(|(i, (_, x))| ((i + 1) as i64) * x).sum();
    println!("Part 1: {result}");

    hands_and_bets.sort_by(|a, b| part_2_cmp(a.0, b.0));
    let result: i64 = hands_and_bets.iter().enumerate().map(|(i, (_, x))| ((i + 1) as i64) * x).sum();
    println!("Part 2: {result}");
}

#[test]
fn comparisons() {
    // test the five categories
    assert_eq!(hand_score("AAAAA"), 5);
    assert_eq!(hand_score("AABAA"), 4);
    assert_eq!(hand_score("AABBA"), 3);
    assert_eq!(hand_score("CCCTK"), 2);
    assert_eq!(hand_score("TTCCO"), 1);
    assert_eq!(hand_score("FFMLO"), 0);
    assert_eq!(hand_score("A2345"), -1);

    // test the single-card orders
    assert_eq!(cmp("A", "K"), Ordering::Greater);
    assert_eq!(cmp("K", "Q"), Ordering::Greater);
    assert_eq!(cmp("Q", "J"), Ordering::Greater);
    assert_eq!(cmp("J", "T"), Ordering::Greater);
    assert_eq!(cmp("T", "9"), Ordering::Greater);
    assert_eq!(cmp("9", "8"), Ordering::Greater);
    assert_eq!(cmp("8", "7"), Ordering::Greater);
    assert_eq!(cmp("7", "6"), Ordering::Greater);
    assert_eq!(cmp("6", "5"), Ordering::Greater);
    assert_eq!(cmp("5", "4"), Ordering::Greater);
    assert_eq!(cmp("4", "3"), Ordering::Greater);
    assert_eq!(cmp("3", "2"), Ordering::Greater);

    // test the final category
    assert_eq!(cmp("5436", "5432"), Ordering::Greater);
}