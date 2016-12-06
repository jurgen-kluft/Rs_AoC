use std::error::Error;
use std::path::Path;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn aoc_main(run: bool) {
    if run == false {
        return;
    }

    let path = Path::new("day3_input.txt");
    let path_str = path.display();


    let input_file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", path_str, why.description()),
        Ok(input_file) => input_file,
    };

    let mut column0 : Vec<i32> = Vec::new();
    let mut column1 : Vec<i32> = Vec::new();
    let mut column2 : Vec<i32> = Vec::new();
    let column_null : Vec<i32> = Vec::new();

    let mut num_good_triangles = 0;
    let buffered_file1 = BufReader::new(&input_file);
    for (_, line) in buffered_file1.lines().enumerate() {
        let triangle = line.unwrap();
        
        let edges : Vec<&str> = triangle.split(' ').collect();
        let mut iarray: [i32; 3] = [0, 1, 2];
        let mut icolumn : usize = 0;
        for e in &edges {
            let e_trimmed = e.trim();
            if e_trimmed.len() > 0 {
                let value : i32 = e_trimmed.parse().unwrap();
                iarray[icolumn] = value;
                match icolumn {
                    0 => column0.push(value),
                    1 => column1.push(value),
                    2 => column2.push(value),
                    _ => println!("error"),
                }
                icolumn += 1;
            }
        }

        println!("{}[{}:{}:{}]", edges.len(), iarray[0], iarray[1], iarray[2]);

        if (iarray[0] + iarray[1]) > iarray[2] {
            if (iarray[0] + iarray[2]) > iarray[1] {
                if (iarray[1] + iarray[2]) > iarray[0] {
                    num_good_triangles += 1;
                }
            }
        }
    }
    println!("Part I : number of good triangles = {}", num_good_triangles);

    num_good_triangles = 0;
    for icolumn in 0..3 {
        let mut column : &Vec<i32>;
        match icolumn {
            0 => column = &column0,
            1 => column = &column1,
            2 => column = &column2,
            _ => column = &column_null,
        }
        
        let mut irow = 0;
        while irow < (column.len()-2) {
            let edge0 = column[irow+0];
            let edge1 = column[irow+1];
            let edge2 = column[irow+2];
            if (edge0 + edge1) > edge2 {
                if (edge0 + edge2) > edge1 {
                    if (edge1 + edge2) > edge0 {
                        num_good_triangles += 1;
                    }
                }
            }
            irow += 3;
        } 
    }


    println!("Part II : number of good triangles = {}", num_good_triangles);

    // `input_file` goes out of scope, and the "hello.txt" input_file gets closed
}

