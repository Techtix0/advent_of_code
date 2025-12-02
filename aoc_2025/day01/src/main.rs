use std::fs;

fn main() {
    //read line per line
    let input: String = fs::read_to_string("./input.txt").expect("Error reading file.");

    let mut pos: u8 = 50;
    let mut i: usize = 0;

    for rotation in input.split("\n") {
        i += 1;
        println!("line: {i}");
        //if L then subtract
        //if R then add
        let value: &u8 = &rotation[1..].parse::<u8>().unwrap();

        if value >= 100 {
            value = value % 100;
        }

        match &rotation.chars().nth(0).unwrap() {
            //get number from input
            'L' =>  if value < &pos {
                pos -= value;
            } else { 
                pos = 100 - (value - pos);
            },

            'R' => pos += value,
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

