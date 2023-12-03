// PART ONE

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let path = "input.txt";
    let colours = [("12", "red"), ("13", "green"), ("14", "blue")].iter().cloned().collect()


    match File::open(&path) {
        Ok(input) => {
            let buffered = io::BufReader::new(input);
            let pattern = Regex::new(r"(\d+)\s+(red|green|blue)").unwrap();
            let mut result = 0;
            let mut game_count = 1;

            for line in buffered.lines() {
                match line {
                    Ok(ln) => {
                        for matched in pattern.captures_iter(ln) {
                            println!("Mathed_set: {}", matched);

                            let number: i32 = matched[1].parse().unwrap();
                            let mut valid_set = 1;

                            if number < 12 {
                                continue
                            }
                            if number > 15 {
                                valid_set = 0;
                                break
                            }
                            if number == 13 {
                            }
                        }
                        if valid_set == 1 {
                            result = result + game_count;
                        }
                        game_count += 1;
                    },
                Err(e) => eprintln!("Line is broken :( {}", e),
                }            
            }
        }
        Err(e) => eprintln!("No file for u :( '{}'", e),
    }
}