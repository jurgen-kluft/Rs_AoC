extern crate num;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
enum MapDirection
{
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, PartialEq)]
struct MapLoc {
    x: i32,
    y: i32,
}

fn turn_left(dir: MapDirection) -> MapDirection {
    match dir {
	    MapDirection::North => return MapDirection::West,
        MapDirection::East => return  MapDirection::North, 
        MapDirection::South => return MapDirection::East,
        MapDirection::West => return  MapDirection::South
    }
}

fn turn_right(dir: MapDirection) -> MapDirection {
    match dir {
	    MapDirection::North => return MapDirection::East,
        MapDirection::East => return  MapDirection::South, 
        MapDirection::South => return MapDirection::West,
        MapDirection::West => return  MapDirection::North
    }
}

fn map_move(dir: MapDirection, loc: MapLoc, steps: i32) -> MapLoc {
    match dir {
	    MapDirection::North => return MapLoc { x: loc.x, y: loc.y + steps},
        MapDirection::East => return  MapLoc { x: loc.x + steps, y: loc.y}, 
        MapDirection::South => return MapLoc { x: loc.x, y: loc.y - steps},
        MapDirection::West => return  MapLoc { x: loc.x - steps, y: loc.y},
    }
}

fn is_move_left(str: &str) -> bool {
    return str.trim().starts_with("L");
}

fn is_move_right(str: &str) -> bool {
    return str.trim().starts_with("R");
}

fn parse_steps(str: &str) -> i32 {
    let instr_str : String = str.to_string();
    let steps_str = instr_str.trim_matches(|c| c == 'L' || c == 'R' || c == ' ');
    println!("{}", steps_str);
    let steps: i32 = FromStr::from_str(&steps_str).unwrap();
    return steps;
}

pub fn aoc_main() {
    let path = Path::new("input.txt");
    let path_str = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut input_file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path_str, why.description()),
        Ok(input_file) => input_file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut instructions_str = String::new();
    match input_file.read_to_string(&mut instructions_str) {
        Err(why) => panic!("couldn't read {}: {}", path_str, why.description()),
        Ok(_) => print!("{} contains:\n{}", path_str, instructions_str),
    }

    // Split the string into pieces into a Vector
    let instructions_vec: Vec<&str> = instructions_str.split(',').collect();

    let mut loc = MapLoc { x: 0, y : 0 };
    let mut dir = MapDirection::North;

    for instruction_str in &instructions_vec {
        if is_move_left(instruction_str) {
            dir = turn_left(dir);
        } else if is_move_right(instruction_str) {
            dir = turn_right(dir);
        }

        let steps = parse_steps(instruction_str);

        println!("direction: {}, steps: {}", match dir {
             MapDirection::North => "North",
             MapDirection::East => "East",
             MapDirection::South => "South",
             MapDirection::West => "West",
            }, steps);

        loc = map_move(dir, loc, steps);
    }
    
    println!("{}:{}", loc.x, loc.y);

    // `input_file` goes out of scope, and the "hello.txt" input_file gets closed
}
