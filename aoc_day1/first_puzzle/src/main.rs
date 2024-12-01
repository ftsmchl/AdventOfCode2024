use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use num::abs;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage {} <path of the input file>", args[0]);
        return;
    }

    let file_path = &args[1];
    println!("file path input is {file_path}");
    
    // Open the file
    let file = File::open(file_path).unwrap_or_else(|error| {
        eprintln!("Error opening file {}: {}", file_path, error);
        std::process::exit(1);
    });

    // Create a buffered reader for the file
    let reader = io::BufReader::new(file);

    let first_coordinates = &mut Vec::new();
    let second_coordinates = &mut Vec::new();

    // Read the file line by line
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                let words: Vec<&str> = content.split_whitespace().collect();
                let first: i32 = match words[0].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        eprintln!("First coordinate not an i32: {}", err);
                        std::process::exit(1);
                    }
                };
                let second: i32 = match words[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        eprintln!("Second coordinate not an i32: {}", err);
                        std::process::exit(1);
                    }
                };

                first_coordinates.push(first);
                second_coordinates.push(second);

            },
            Err(error) => eprintln!("Line {} could not be read: {}", index + 1, error),
        }
    }

    let mut sum_diff = 0;

    first_coordinates.sort();
    second_coordinates.sort();
    
    for (index, first_word) in first_coordinates.iter().enumerate() {
        let second_word = second_coordinates[index];
        let absolute_diff = abs(first_word - second_word);
        sum_diff += absolute_diff;
    }

    println!("sum_diff = {sum_diff}");
   
}





















