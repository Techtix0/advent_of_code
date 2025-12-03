use std::fs;

struct Rotation {
    direction: char, 
    value: usize,
}

fn main() {
    //read line per line
    let input: String = fs::read_to_string("./input.txt").expect("Error reading file.");

    let mut pos: usize = 50;

    for rotation in input.split("\n") {
        let rot: Rotation = Rotation { 
            direction: rotation.chars().nth(0).unwrap(), 
            value: &rotation[1..].parse::<usize>().unwrap() % 100
        };


        match &rotation.chars().nth(0).unwrap() {
            //get number from input
            'L' =>  if rot.value < pos {
                pos -= rot.value;
            } else {
                pos = 100 - (rot.value - pos);
            },

            'R' => pos += rot.value,
            _ => println!("Error reading value"),
        }

        if pos >= 100 {
            pos = pos % 100;
        }
        
        println!("{pos}");

        //take module of result so that 100 => 0
        //add total of 0's to result
        //print result
        
    }
}

