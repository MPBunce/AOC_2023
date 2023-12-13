use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::vec;

use crate::models::Hand;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_one() -> io::Result<()> {
    let lines = read_lines("./src/small.txt");

    let mut hands: Vec<Hand> = Vec::new();

    for n in lines {
        let res: Vec<_> = n.split_ascii_whitespace().collect();
        let temp = Hand {
            cards: res[0].to_string(),
            bid: res[1].parse::<i32>().unwrap(),
        };
        hands.push(temp);
    }

    for hand in &hands{
        println!("{:?}", hand);
    }

    bubble_sort(&mut hands);

    for hand in hands{
        println!("{:?}", hand);
    }

    Ok(())
}

fn bubble_sort(arr: &mut Vec<Hand>) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if winning_hand(&arr[j].cards , &arr[j + 1].cards){
                arr.swap(j, j + 1);
            }
        }
    }
}

fn winning_hand(hand_one: &String, hand_two: &String) -> bool {
    let h1 = hand_one;
    let h2 = hand_two;

    let mut h1_char = HashMap::new();
    for ch in h1.chars() {
        // Use entry API to insert the character if it doesn't exist, and update its count
        *h1_char.entry(ch).or_insert(0) += 1;
    }

    let mut h2_char = HashMap::new();
    for ch in h2.chars() {
        // Use entry API to insert the character if it doesn't exist, and update its count
        *h2_char.entry(ch).or_insert(0) += 1;
    }

    println!("Hand One: {:?} Length: {:?}, Hand Two: {:?} Length {:?}", h1, h1_char.len(), h2, h2_char.len() );

    if h1_char.len() < h1_char.len() {
        return true
    } else if h1_char.len() == h1_char.len() {
        

        
    } else{
        return false        
    }


}