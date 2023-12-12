use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
            if arr[j].bid > arr[j + 1].bid {
                arr.swap(j, j + 1);
            }
        }
    }
}
