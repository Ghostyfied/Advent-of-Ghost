// PART ONE

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let path = "input_01.txt";
    let max_red = 12
    let max_green = 13
    let max_blue = 14

    match File::open(&path) {
        Ok(input) => {
            let buffered = io::BufReader::new(input);

            for line in buffered.lines() {
                match line {
                    Ok(ln) => {
                        println!("{}", ln);
                    },
                Err(e) => eprintln!("Line is broken :( {}", e),
                }            
            }
        }
    }
}