use std::fs::read_to_string;
use std::fs::File;
use std::io;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_two() -> io::Result<()> {
    let lines = read_lines("./src/small.txt");

    for n in lines{
        println!("{:?}", n);
    }
    
    
    Ok(())
}

