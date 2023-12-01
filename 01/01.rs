// PART TWO

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let path = "input_01.txt";

    match File::open(&path) {
        Ok(input) => {
            let buffered = io::BufReader::new(input);

            let number_mapping = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)].iter().cloned().collect::<HashMap<_, _>>();

            let mut sum: i32 = 0;

            for line in buffered.lines() {
                let mut x = None;
                let mut y = None;
                let mut x_index = usize::MAX;
                let mut y_index = 0;

                match line {
                    Ok(ln) => {
                        for (word, &number) in &number_mapping {
                            if let Some(i) = ln.find(word) {
                                if i < x_index {
                                    x_index = i;
                                    x = Some(number);
                                }
                            }
                            if let Some(j) = ln.rfind(word) {
                                if j >= y_index {
                                    y_index = j;
                                    y = Some(number);
                                }
                            }
                        }

                        if let (Some(first), Some(last)) = (x, y) {
                            let final_number_str = format!("{}{}", first, last);
                            if let Ok(final_number) = final_number_str.parse::<i32>() {
                                sum += final_number;
                            }
                        }
                    },
                Err(e) => eprintln!("Line is broken :( {}", e),
                }
            }

            println!("Final result: {}", sum);
        },
        Err(e) => eprintln!("No file for u :( '{}': {}", path, e),
    }
}

// PART ONE

// use std::fs::File;
// use std::io::{self, BufRead};

// fn main() {
//     let path = "input_01.txt";

//     match File::open(&path) {
//         Ok(input) => {
//             let buffered = io::BufReader::new(input);

//             let mut sum: i32 = 0;
//             for line in buffered.lines() {
//                 match line {
//                     Ok(ln) => {
//                         let numbers: String = ln.chars().filter(|c| c.is_numeric()).collect();
//                         let first_char: Option<char> = numbers.chars().next();
//                         let last_char: Option<char> = numbers.chars().last();
//                         let mut final_string = String::new();

//                         if let Some(c) = first_char {
//                             final_string.push(c);
//                         }
//                         if let Some(c) = last_char {
//                             final_string.push(c);
//                         }

//                         let final_number = final_string.parse::<i32>().unwrap();
//                         sum = sum + final_number;
//                     },
//                     Err(e) => eprintln!("Line is broken :( {}", e),
//                 }
//             }
//             println!("Final result: {}", sum);
//         },
//         Err(e) => eprintln!("No file for u :( '{}': {}", path, e),
//     }
// }