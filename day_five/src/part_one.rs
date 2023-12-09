use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn pt_one () -> std::io::Result<()> {

    let mut sum: i32 = 0;
    // Open the file for reading
    let file = File::open("./src/small.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {

        let line = line_result?;
        println!("{}", line);

    }

    Ok(())
}
