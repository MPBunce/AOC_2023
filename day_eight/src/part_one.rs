use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_one() -> io::Result<()> {

    

    let lines = read_lines("./src/small.txt");

    let instructions = &lines[0];
    println!("{:?}", instructions);

    for i in 2..lines.len() {
        let n = &lines[i];
        println!("{:?}", n);
    }
    
    Ok(())
}

