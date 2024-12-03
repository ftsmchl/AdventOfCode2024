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
                let mut previous_num:i32 = 0;
                let mut diff_symbol:i32 = 0;
                let last_report_index = report.len() - 1;              
                let mut found_a_not_eligible_level:bool = false;
                let mut diff_symbol_calculated:bool = false;

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
                            let diff = report_level_int - previous_num;
                            if 0 < abs(diff) && abs(diff) <= 3 {
                                diff_symbol = report_level_int - previous_num;
                                diff_symbol_calculated = true;
                                previous_num = report_level_int; 
                            }else{
                                found_a_not_eligible_level = true;
                            }
                        },
                        i if i == last_report_index => {
                            if !diff_symbol_calculated {
                                diff_symbol = report_level_int - previous_num;
                            }
                            let diff = report_level_int - previous_num;
                            if 0 < abs(diff) && abs(diff) <=3 && diff * diff_symbol > 0 {
                                safe_reports += 1; 
                            }else{
                                if !found_a_not_eligible_level {
                                    safe_reports += 1; 
                                }else{
                                    break;
                                }
                            }

                        },
                        _ => {
                            if !diff_symbol_calculated {
                                diff_symbol = report_level_int - previous_num;
                            }
                            let diff = report_level_int - previous_num;
                            if 0 < abs(diff) && abs(diff) <=3 && diff * diff_symbol > 0 {
                                previous_num = report_level_int;
                            }else{
                                if !found_a_not_eligible_level {
                                    found_a_not_eligible_level = true;
                                }else{
                                    break;
                                }
                            }

                        }
                    }
                

                }
            },
            Err(error) => eprintln!("Could not read line with index {} and err: {}", index + 1, error)
        }
    }
        println!("Safe reports that can tolerate a single bad lever are {}", safe_reports)

}





























