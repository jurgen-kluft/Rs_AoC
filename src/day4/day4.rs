extern crate regex;

use std::error::Error;
use std::path::Path;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use regex::Regex;

pub fn aoc_main(run: bool) {
    if run == false {
        return;
    }

    let path = Path::new("day4_input.txt");
    let path_str = path.display();


    let input_file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", path_str, why.description()),
        Ok(input_file) => input_file,
    };


    let buffered_file1 = BufReader::new(&input_file);
    for (_, line) in buffered_file1.lines().enumerate() {
        let l = line.unwrap();
        
        // 
        let re = Regex::new(r"^([a-z]+)(-[a-z]+){2,4}-([0-9]+)\[[a-z]+\]$").unwrap();

        println!("{}", l);
    }

    // `input_file` goes out of scope, and the "hello.txt" input_file gets closed
}

