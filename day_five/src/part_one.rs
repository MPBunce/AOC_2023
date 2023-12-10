use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use std::vec;

use crate::models::Map_Range;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_one() -> io::Result<()> {

    let lines = read_lines("./src/small.txt");

    let seeds = &lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("seeds: {:?}\n", &seeds);
    let mut dic_num: i32 = 0;
    let mut map_name = "";

    let mut maps_vector: Vec<Map_Range> = Vec::new();
    let mut my_map_range = Map_Range {
        map_name: "tst",
        map_lists: Vec::new()
    };

    for line in &lines[1..] {

        if line.is_empty(){      
            continue;
        } else {
            if line.contains("map:") {
                maps_vector.push(my_map_range.clone() );
                dic_num += 1;
                let temp_name: Vec<_> = line.split(" map:").collect();
                map_name = temp_name[0];
                my_map_range.map_name = &map_name;
                my_map_range.map_lists = Vec::new();
                continue;
            } else {
                let values: Vec<usize> = line
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().expect("Failed to parse usize"))
                    .collect();
                my_map_range.map_lists.push(values);
            }
        }
    }

    let vector_len = maps_vector.len();
    let mut locations = seeds.clone();

    for i in 1..vector_len {
        let value = &maps_vector[i];

        for n in &value.map_lists {
            let temp = n.clone();
            let mut destination = temp[0];
            let mut source = temp[1];
            let mut range = temp[2];

            let mut destination_range: Vec<usize> = Vec::new();
            let mut source_range: Vec<usize> = Vec::new();

            destination_range.push(destination);
            source_range.push(source);

            for i in 0..(range -1){
                destination += 1;
                destination_range.push(destination);
            }

            for i in 0..(range -1){
                source += 1;
                source_range.push(source);
            }

            // println!("Desination {:?}", destination_range);
            // println!("Source range {:?}", source_range);

            let len_seeds = locations.len();
            for i in 0..len_seeds{
                let temp = locations[i];
                if let Some(position) = source_range.iter().position(|&x| x == temp) {
                    locations[i] = destination_range[position];

                } else {
                    continue;
                }
            }



        }
        println!("location: {:?}", locations);
    }


    Ok(())
}