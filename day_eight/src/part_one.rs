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

    let char_vec: Vec<char> = instructions.chars().collect();
    let mut char_map: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        let n = &lines[i];
        println!("{:?}", n);

        let res: Vec<_> = n.split(&[' ', ',', '(', ')'][..]).collect();
        let new_key = res[0].to_string(); 
        let new_tuple: (String, String) = (res[3].to_string(), res[5].to_string());
    
        char_map.insert(new_key, new_tuple);
    
    }

    println!("{:?}", char_vec);
    for n in char_map {
        println!("{:?}", n)
    }

    let current_chars = "AAA".to_string();
    let da_length = char_vec.len();
    println!("{:?}, ", da_length);

    let mut count:i32 = 0;


    for i..da_length {


    }

    Ok(())

}

