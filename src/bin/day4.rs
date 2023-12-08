use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::{min};

fn main() {
    let file_data = read_to_string("src/bin/day4.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let mut total = 0;
    let mut matches: Vec<i32> = vec![];
    for line in lines {
        let scratchcard = line.split(':').collect::<Vec<_>>()[1];
        let scratchcard_sides = scratchcard.split('|').collect::<Vec<_>>();
        let (mut left_card_strs, mut right_card_strs): (Vec<&str>, Vec<&str>) =
            (scratchcard_sides[0]
              .trim()
              .split(' ')
              .collect(),
          scratchcard_sides[1]
              .trim()
              .split(' ')
              .collect());
        left_card_strs.retain(|x| &*x.trim() != "");
        right_card_strs.retain(|x| &*x.trim() != "");
        let left_card_ints: Vec<_> = left_card_strs.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        let right_card_ints: Vec<_> = right_card_strs.iter().map(|s| s.parse::<i32>().unwrap()).collect();

        let mut left_heap = BinaryHeap::new();
        let mut right_heap = BinaryHeap::new();
        for card in left_card_ints { left_heap.push(card); }
        for card in right_card_ints { right_heap.push(card); }

        let mut matching = 0;
        while (!left_heap.is_empty()) && (!right_heap.is_empty()) {
            let left_number = left_heap.pop().unwrap();
            // println!("Left {left_number}");
            while (!right_heap.is_empty()) && (left_number < *right_heap.peek().unwrap()) {
                // println!("\tRight {}", right_heap.peek().unwrap());
                right_heap.pop();
            }
            if (!right_heap.is_empty()) && (left_number == *right_heap.peek().unwrap()) {
                right_heap.pop();
                matching += 1;
            }
        }
        matches.push(matching);
        if matching > 0 {total += 2_i32.pow((matching - 1) as u32) }
    }
    println!("Part 1: {total}");

    /* Part 2 */
    total = 0;
    let mut multipliers = vec![1; matches.len()];
    for i in 0..matches.len() {
        let this_matching = matches[i];
        for j in i + 1..min(1 + i + (this_matching  as usize), matches.len()) {
            multipliers[j] += multipliers[i];
        }
        total += multipliers[i];
    }
    println!("Part 2: {total}");
}