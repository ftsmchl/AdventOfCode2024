use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage {} <your argument>", args[0]);
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
    let mut second_coordinates_map = HashMap::new();


    //for (index, line) in reader.lines().enumerate() {
    for  line in reader.lines() {
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

                // Add the second word to the hashmap and increment the count
                match second_coordinates_map.get(&second) {
                    Some(num) => second_coordinates_map.insert(second, num + 1),
                    None => second_coordinates_map.insert(second,1),
                };

            },
            Err(err) => {
             eprintln!("Err in reading line {err}");   
            },
        }
    }

    let mut sum_appeareances: i32 = 0;

    for first_coordinate in first_coordinates {
        match second_coordinates_map.get(&first_coordinate){
            Some(times) => sum_appeareances += *first_coordinate * times,
            None => {},  
        }
    }

    println!("Sum is: {sum_appeareances}");


}




















