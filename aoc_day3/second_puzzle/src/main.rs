use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use num::abs;
//
//

struct DoDont{
    line: u32,
    index: u32,
}


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
    

    let mut previous_do_index:DoDont = DoDont{line: 0, index: 0};
    let mut previous_dont_index:DoDont = DoDont{line: 0,index: 0};

    //let mut undisputed_index = -1;

    //let mut safe_reports:i32 = 0;
    for (line_index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {

                //if line_index == 1 {
                 //       println!("xairetw sas");
                  //      std::process::exit(1);
                //}

                let v: Vec<_> = content.match_indices("mul(").collect();
                let dos: Vec<_> = content.match_indices("do()").collect();
                let donts: Vec<_> = content.match_indices("don't()").collect();
                let char_vec: Vec<char> = content.chars().collect();

                //println!("ta dos = {dos:?}");
                //println!("ta donts = {donts:?}");
                //println!("to v  = {v:?}");

                let vector_len = v.len() - 1;

                let mut current_do_index = 0;
                let mut current_dont_index = 0;

                let mut current_do_offset = 0;
                let mut current_dont_offset = 0;

                let mut do_exists_cur_line = false;
                let mut dont_exists_cur_line = false;

                let mut max_do_and_dont_indexes_calculated = false;

                let mut index_offset = 0;

                for (index, _) in &v {

                    if line_index == 0 && !do_exists_cur_line{
                        println!("To ekana true to do_exists...");
                        do_exists_cur_line = true;
                        current_do_index = 0;
                    }

                    for i in current_do_offset..(dos.len()) {
                        let (do_index, _) = &dos[i];
                        if do_index > index && i != 0  {
                            current_do_offset = i - 1 ;
                            let (tmp_do_index, _) = &dos[i - 1];
                            current_do_index = *tmp_do_index;
                            do_exists_cur_line = true;
                            break;
                        }
                        
                        if do_index > index && i == 0 {
                            break;
                        }

                        if do_index < index {
                            current_do_offset = i;
                            current_do_index = *do_index;
                            do_exists_cur_line = true;
                        }
                    }
                    
                    //Find the closest don't index
                    for j in current_dont_offset..(donts.len()) {
                        let (dont_index, _) = &donts[j];
                        if dont_index > index && j != 0 {
                            println!("i read dont index = {} while index = {}", dont_index, index);
                            current_dont_offset = j - 1 ;
                            let (tmp_dont_index, _) = &donts[j - 1];
                            current_dont_index = *tmp_dont_index;
                            dont_exists_cur_line = true;
                            break;
                        }
                        
                        if dont_index > index && j == 0 {
                            break;
                        }

                        if dont_index < index {
                            println!("i read dont index = {} while index = {}", dont_index, index);
                            current_dont_offset = j;
                            current_dont_index = *dont_index;
                            dont_exists_cur_line = true;
                            //break;
                        }
                    }

                    // If i am in the last 
                    //
                    if index_offset == vector_len  {
                        //println!("bhka sto last");

                        if dos.len() > 0 {
                            let (temp_do_index, _) = &dos[dos.len() - 1];
                            previous_do_index.line = line_index as u32;
                            previous_do_index.index = *temp_do_index as u32;
                        }

                        if donts.len() > 0 {
                            let (temp_dont_index, _) = &donts[donts.len() - 1];
                            previous_dont_index.line = line_index as u32;
                            previous_dont_index.index = *temp_dont_index as u32;
                        }
                    }

                    index_offset +=1;

                    if !do_exists_cur_line && !dont_exists_cur_line  {

                            println!("den eimai edw ");

                            if previous_dont_index.line > previous_do_index.line {

                                continue;
                            }

                            if (previous_dont_index.line == previous_do_index.line) && (previous_dont_index.index > previous_do_index.index) {

                                continue;
                            }

                        }

                    /*
                        // If i am in the last 
                        //
                        if index_offset == vector_len  {
                            //println!("bhka sto last");

                            if dos.len() > 0 {
                                let (temp_do_index, _) = &dos[dos.len() - 1];
                                previous_do_index.line = line_index as u32;
                                previous_do_index.index = *temp_do_index as u32;
                            }

                            if donts.len() > 0 {
                                let (temp_dont_index, _) = &donts[donts.len() - 1];
                                previous_dont_index.line = line_index as u32;
                                previous_dont_index.index = *temp_dont_index as u32;
                            }
                        }
                    */


                        let mut dist_from_do = 0;
                        let mut dist_from_dont = 0;

                        if do_exists_cur_line {
                            println!("index = {} , current_do = {}", index, current_do_index);
                             dist_from_do = if *index > current_do_index {
                                *index - current_do_index
                            }else{
                                println!("giati eimai edw");
                                current_do_index - index
                            };
                        }
                        println!("dist_from_do= {}", dist_from_do);

                        if dont_exists_cur_line{
                            println!("index = {} , current_dont = {}", index, current_dont_index);
                            dist_from_dont = if *index > current_dont_index {
                                *index - current_dont_index
                            }else{
                                println!("giati eimai edw");
                                current_dont_index - index
                            };
                        }
                        println!("dist_from_dont= {}", dist_from_dont);

                        if (do_exists_cur_line && dont_exists_cur_line) && (dist_from_dont < dist_from_do) {
                            continue;
                        }

                        if dont_exists_cur_line && !do_exists_cur_line {
                            continue;
                        }


                        //index_offset +=1 ;
                        

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
                    println!("(first num, second num) = ({},{})", first_number, second_number);

                    multiplication_sum += multiplication;


                }
            },
            Err(err) => eprintln!("line in index {} error: {}", line_index + 1, err)
        }
    }

    println!("multiplication sum = {multiplication_sum}");



}
