use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "../data/adventofcode.com_2023_day_1_input.txt";
    // let file_path = "../data/day1test.txt";
    println!("In file {}", file_path);
    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let map = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);

    for (key, value) in map {
        // println!("replacing {} by {}", key, value     );
        contents = contents.replace(key, value);
    }
    // println!(" before {}", contents);
    // contents = contents.replace("one", "one1one");
    // println!("after {}", contents);

    let contents_split = contents.split("\n");
    let mut counter = 0;
    for line in contents_split {
        let mut vec = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                vec.push(char.to_digit(10).expect("couln't decode digit!"));
            }
        }

        if vec.len() >= 1 {
            let num = vec[0] * 10 + vec.last().expect("no nums");
            println!("num is is {:?}", num);
            counter += num;
        }
    }
    println!("Num is: {}", counter)
}
