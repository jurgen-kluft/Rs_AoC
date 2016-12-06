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

// fn map_move(dir: MapDirection, loc: MapLoc, steps: i32) -> MapLoc {
//     match dir {
// 	    MapDirection::North => return MapLoc { x: loc.x, y: loc.y + steps},
//         MapDirection::East => return  MapLoc { x: loc.x + steps, y: loc.y}, 
//         MapDirection::South => return MapLoc { x: loc.x, y: loc.y - steps},
//         MapDirection::West => return  MapLoc { x: loc.x - steps, y: loc.y},
//     }
// }

fn find_dup_loc(loc: MapLoc, history: &mut Vec<MapLoc>) -> (bool, MapLoc) {
    for l in history {
        if loc.x == l.x && loc.y == l.y {
            return (true, MapLoc {x:l.x, y:l.y});
        }
    }
    return (false, MapLoc {x:0, y:0});
}

fn map_move_with_history(dir: MapDirection, loc: MapLoc, steps: i32, history: &mut Vec<MapLoc>) -> (MapLoc, bool, MapLoc) {
    let mut dx = 0;
    let mut dy = 0;
    match dir {
	    MapDirection::North => dy =  1,
        MapDirection::East =>  dx =  1,
        MapDirection::South => dy = -1,
        MapDirection::West =>  dx = -1,
    }

    let mut l = MapLoc{ x: loc.x, y: loc.y };
    let mut n = steps;
    let mut f = false;              // found duplicate ?
    let mut d = MapLoc {x:0, y:0};  // the duplicate location that was found
    while n > 0 
    { 
        l.x = l.x + dx; 
        l.y = l.y + dy; 
        if f == false {
            let (tf,td) = find_dup_loc(l, history);
            if tf == true && f == false {
                f = tf;
                d = td;
            }
        }
        history.push(l); 
        n -= 1; 
    }

    return (history[history.len() - 1], f, d);
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
    let steps: i32 = FromStr::from_str(&steps_str).unwrap();
    return steps;
}


pub fn aoc_main() {
    let path = Path::new("day1_input.txt");
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

    let start = MapLoc { x: 0, y : 0 };
    let mut loc = MapLoc { x: start.x, y : start.y };
    let mut dir = MapDirection::North;

    let mut locations_vec: Vec<MapLoc> = Vec::new();
    locations_vec.push(start);

    let mut loc_dup = false; 
    let mut dup_loc = MapLoc { x: start.x, y : start.y }; 

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

        let (tloc, tloc_dup, tdup_loc) = map_move_with_history(dir, loc, steps, &mut locations_vec);
        loc = tloc;
        if tloc_dup == true && loc_dup == false {
            dup_loc = tdup_loc;
            loc_dup = true;
        }

        locations_vec.push(loc);
    }
    
    println!("HQ-1 {}:{}", loc.x, loc.y);
    if loc_dup == true {
        println!("HQ-2 {}:{}", dup_loc.x, dup_loc.y);
    }

    // `input_file` goes out of scope, and the "hello.txt" input_file gets closed
}
