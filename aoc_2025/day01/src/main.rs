use std::fs;

fn main() {
    //read line per line
    let input: String = fs::read_to_string("./input.txt").expect("Error reading file.");

    let mut pos: u8 = 50;

    for rotation in input.split("\n") {
        //if L then subtract
        //if R then add
        match &rotation.chars().nth(0).unwrap() {
            //get number from input
            'L' => pos -= &rotation.chars().nth().unwrap(),
            'R' => pos += 1,
            _ => println!("Error"),
        }

        if pos == 100 {
            pos = 0;
        }

        println!("{pos}");

        //take module of result so that 100 => 0
        //add total of 0's to result
        //print result
        
    }

    
}
