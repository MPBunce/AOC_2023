use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_two () -> std::io::Result<()> {
    
    let mut lines = read_lines("./src/small.txt");

    let mut seeds = &lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("seeds: {:?}", &seeds);

    for n in seeds {
        for line in &lines[2..] {
            println!("seed: {:?} lines: {:?}", n, &line);
        }
    }

    Ok(())

}
