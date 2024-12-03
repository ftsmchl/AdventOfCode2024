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

    let mut safe_reports:i32 = 0;
    
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                let report: Vec<&str> = content.split_whitespace().collect();
                let mut previous_num: i32 = 0;
                let mut diff_symbol: i32 = 0;
                let last_report_index = report.len() - 1;              

                for (report_index, report_level) in report.iter().enumerate() {


                    let report_level_int: i32 = match report_level.trim().parse(){
                        Ok(num) => num,
                        Err(err) => {
                            eprintln!("report not an i32: {}", err);
                            std::process::exit(1);
                        }
                    };

                    match report_index {
                        0 => {
                            previous_num = report_level_int;

                        },
                        1 => {
                            // Check if current diffs from previous at least one no more than three
                            diff_symbol = report_level_int - previous_num;
                            if 0 < abs(diff_symbol) && abs(diff_symbol) <= 3 {
                                previous_num = report_level_int; 
                            }else{
                                break;
                            }

                        },
                        i if i == last_report_index => {
                            let diff = report_level_int - previous_num;
                            if 0 < abs(diff) && abs(diff) <=3 && diff * diff_symbol > 0 {
                                safe_reports += 1;
                            }else{
                                break;
                            }

                        },
                        _ => {
                            let diff = report_level_int - previous_num;
                            if 0 < abs(diff) && abs(diff) <=3 && diff * diff_symbol > 0 {
                                previous_num = report_level_int;
                            }else{
                                break;
                            }

                        }
                    }


                }
            },
            Err(error) => eprintln!("Error occured in line: {} --> err = {}", index + 1, error)
        }
    }

    println!("safe reports are {safe_reports}");


}

