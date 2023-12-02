use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn pt_one () -> std::io::Result<()> {

    let red_max: i32 = 12;
    let green_max: i32 = 13;
    let blue_max: i32 = 14;

    // Open the file for reading
    let file = File::open("./src/small.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {
        // Handle errors
        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                continue; // Skip to the next iteration
            }
        };

        println!("{}", line);

        let games: Vec<&str> = line.split(';').collect();

        println!("{:?}", &games);

        // Iterate over each game
        for game in games {
            if let Some(game_number) = game.split(':').next() {
                if let Ok(number) = game_number.trim().parse::<usize>() {
                    // Extract color and number pairs within each game
                    let pairs: Vec<&str> = game.split(',').collect();
                    for pair in pairs {
                        if let Some((number_str, color)) = pair.split_once(' ') {
                            if let Ok(number) = number_str.trim().parse::<usize>() {
                                println!("Number: {}, Color: {}", number, color.trim());
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\nDone\n\n");

    Ok(())
}
