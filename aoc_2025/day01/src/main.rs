use std::fs;

struct Rotation {
    direction: char, 
    value: usize,
}

fn main() {
    let mut input: String = fs::read_to_string("./input.txt").expect("Error reading file.");
    input.pop();

    let mut pos: usize = 50;
    let mut total: usize = 0;

    for rotation in input.split("\n") {
        let rot: Rotation = Rotation { 
            direction: rotation.chars().nth(0).unwrap(), 
            value: rotation[1..].parse::<usize>().unwrap()
        };


        match rot.direction {
            'L' =>  if rot.value % 100 < pos {
                pos -= rot.value % 100;
            } else {
                pos = 100 - (rot.value % 100 - pos);
            },

            'R' => pos += rot.value,
            _ => println!("Error reading value"),
        }

        if pos >= 100 {
            pos = pos % 100;
        }

        if pos == 0 {
            total += 1;
        }
        
    }
    println!("{total}");
}

