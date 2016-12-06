use std::error::Error;
use std::path::Path;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Copy, Clone, PartialEq)]
struct Key {
    u: i32,
    d: i32,
    l: i32,
    r: i32,

    c: char,
}

struct KeyPad {
    n: i32,
    c: Vec<Key>,
}

pub fn aoc_main(run: bool) {
    if run == false {
        return;
    }

    let path = Path::new("day2_input.txt");
    let path_str = path.display();

    // Keypad layout and navigation data
    // Key starts at digit '5'
    let mut key_pad_1 = KeyPad { n: 5, c : Vec::new() };
    key_pad_1.c.push(Key {u: 0, d: 0, l: 0, r: 0, c:'0'});   // 0
    key_pad_1.c.push(Key {u:-1, d: 4, l:-1, r: 2, c:'1'});   // 1
    key_pad_1.c.push(Key {u:-1, d: 5, l: 1, r: 3, c:'2'});   // 2
    key_pad_1.c.push(Key {u:-1, d: 6, l: 2, r:-1, c:'3'});   // 3
    key_pad_1.c.push(Key {u: 1, d: 7, l:-1, r: 5, c:'4'});   // 4
    key_pad_1.c.push(Key {u: 2, d: 8, l: 4, r: 6, c:'5'});   // 5
    key_pad_1.c.push(Key {u: 3, d: 9, l: 5, r:-1, c:'6'});   // 6
    key_pad_1.c.push(Key {u: 4, d:-1, l:-1, r: 8, c:'7'});   // 7
    key_pad_1.c.push(Key {u: 5, d:-1, l: 7, r: 9, c:'8'});   // 8
    key_pad_1.c.push(Key {u: 6, d:-1, l: 8, r:-1, c:'9'});   // 9

    // Keypad layout and navigation data of part 2
    // Key starts at digit '5'
    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    let mut key_pad_2 = KeyPad { n: 5, c : Vec::new() };
    key_pad_2.c.push(Key {u: 0, d: 0, l: 0, r: 0, c:'0'});   // 0
    key_pad_2.c.push(Key {u:-1, d: 3, l:-1, r:-1, c:'1'});   // 1
    key_pad_2.c.push(Key {u:-1, d: 6, l:-1, r: 3, c:'2'});   // 2
    key_pad_2.c.push(Key {u: 1, d: 7, l: 2, r: 4, c:'3'});   // 3
    key_pad_2.c.push(Key {u:-1, d: 8, l: 3, r:-1, c:'4'});   // 4
    key_pad_2.c.push(Key {u:-1, d:-1, l:-1, r: 6, c:'5'});   // 5
    key_pad_2.c.push(Key {u: 2, d:10, l: 5, r: 7, c:'6'});   // 6
    key_pad_2.c.push(Key {u: 3, d:11, l: 6, r: 8, c:'7'});   // 7
    key_pad_2.c.push(Key {u: 4, d:12, l: 7, r: 9, c:'8'});   // 8
    key_pad_2.c.push(Key {u:-1, d:-1, l: 8, r:-1, c:'9'});   // 9
    key_pad_2.c.push(Key {u: 6, d:-1, l:-1, r:11, c:'A'});   // A
    key_pad_2.c.push(Key {u: 7, d:13, l:10, r:12, c:'B'});   // B
    key_pad_2.c.push(Key {u: 8, d:-1, l:11, r:-1, c:'C'});   // C
    key_pad_2.c.push(Key {u:11, d:-1, l:-1, r:-1, c:'D'});   // D

    // Open the path in read-only mode, returns `io::Result<File>`
    let input_file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path_str, why.description()),
        Ok(input_file) => input_file,
    };

    let mut keycode_1 : String = String::new();
    let mut keycode_2 : String = String::new();

    let buffered_file1 = BufReader::new(&input_file);
    for (_, line) in buffered_file1.lines().enumerate() {
        let l = line.unwrap();
        let mut n1 = key_pad_1.n as usize;
        for c in l.chars() { 
            // do something with `c`
            if c == 'U' {
                let key = key_pad_1.c[n1];
                let nn = key.u;
                if nn != -1 {
                    n1 = nn as usize;
                }
            } else if c == 'D' {
                let nn = key_pad_1.c[n1].d;
                if nn != -1 {
                    n1 = nn as usize;
                }
            } else if c == 'L' {
                let nn = key_pad_1.c[n1].l;
                if nn != -1 {
                    n1 = nn as usize;
                }
            } else if c == 'R' {
                let nn = key_pad_1.c[n1].r;
                if nn != -1 {
                    n1 = nn as usize;
                }
            }
        }
        keycode_1.push(key_pad_1.c[n1].c);

        // Next key should start at 'n'
        key_pad_1.n = n1 as i32;

        // PART 2

        let mut n2 = key_pad_2.n as usize;
        for c in l.chars() { 
            // do something with `c`
            if c == 'U' {
                let key = key_pad_2.c[n2];
                let nn = key.u;
                if nn != -1 {
                    n2 = nn as usize;
                }
            } else if c == 'D' {
                let nn = key_pad_2.c[n2].d;
                if nn != -1 {
                    n2 = nn as usize;
                }
            } else if c == 'L' {
                let nn = key_pad_2.c[n2].l;
                if nn != -1 {
                    n2 = nn as usize;
                }
            } else if c == 'R' {
                let nn = key_pad_2.c[n2].r;
                if nn != -1 {
                    n2 = nn as usize;
                }
            }
        }
        keycode_2.push(key_pad_2.c[n2].c);

        // Next key should start at 'n'
        key_pad_2.n = n2 as i32;
    }  

    println!("keycode-1 : {}", keycode_1);
    println!("keycode-2 : {}", keycode_2);

    // `input_file` goes out of scope, and the "hello.txt" input_file gets closed
}
