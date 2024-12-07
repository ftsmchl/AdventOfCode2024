use std::env;
use std::fs::File;
use std::io::{self, BufRead};


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

    let mut multiplication_sum = 0;

    //let mut safe_reports:i32 = 0;
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {

                let v: Vec<_> = content.match_indices("mul(").collect();
                let char_vec: Vec<char> = content.chars().collect();


                for (index, _) in &v {
                    //First Check if the first number is ok 
                    let mut first_number_index_end = 0;
                    let mut first_num_found = false;
                    let mut first_step = index + 4;

                    loop {
                        match char_vec.get(first_step) {
                            Some(num) => {
                                match num.is_numeric() {
                                    true => {
                                        //first_num_found = true;
                                        first_step += 1;
                                    },
                                    false => {
                                        if *num == ',' {
                                            first_number_index_end = first_step;
                                            first_num_found = true;
                                            break;
                                        }else{
                                            first_num_found = false;
                                            break;
                                        }
                                    }
                                }
                            },
                            None => {

                            }
                        }
                    }

                    if !first_num_found {
                        continue;
                    }

                    let mut first_number:u32 = 0;
                    if let Some(first_num_slice) = content.get(index + 4..first_number_index_end){
                        first_number = match first_num_slice.parse() {
                            Ok(number) => number,
                            Err(err) => {
                                println!("oops {err}");
                                continue;
                            }
                        };
                    }else{
                        println!("Failed to open slice")
                    }


                    let mut second_number_index_end = 0;
                    let mut second_num_found = false;
                    let mut second_step = first_number_index_end + 1;


                    loop {
                        match char_vec.get(second_step) {
                            Some(num) => {
                                match num.is_numeric() {
                                    true => {
                                        second_step += 1;
                                    },
                                    false => {
                                        if *num == ')' {
                                            second_num_found = true;
                                            second_number_index_end = second_step;
                                            break;
                                        }else{
                                            second_num_found = false;
                                            break;
                                        }
                                    }
                                }
                            },
                            None => println!("None")
                        }
                    } // loop ends

                    if !second_num_found{
                        continue;
                    }

                    let mut second_number:u32 = 0;
                    
                    if let Some(second_num_slice) = content.get(first_number_index_end + 1..second_number_index_end){
                        second_number = match second_num_slice.parse() {
                            Ok(number) => number,
                            Err(err) => {
                                println!("oops {err}");
                                continue;
                            }
                        };
                    }else{
                        println!("Failed to open slice")
                    }


                    let multiplication = first_number * second_number;

                    multiplication_sum += multiplication;


                }



            },
            Err(err) => eprintln!("line in index {} error: {}", index+1, err)
        }
    }

    println!("multiplication sum = {multiplication_sum}");



}
