use std::fs::File;
use std::io::{self, BufRead};
use std::mem;
use std::path::Path;

fn main() {
    let mut vec = Vec::new();
    if let Ok(lines) = read("./src/input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let current = data.parse::<i32>().unwrap();
                vec.push(current);
            }
        }
    }
    part1(&vec);

    part2(&vec);
    
}

fn part1(data: &Vec<i32>) {
    let mut increases = 0;
    let mut previous = 0;

    for current in data {
        if current > &previous && previous > 0 {
            increases = increases + 1;
        }
        previous = *current;
    }
    println!("part 1 = {}", increases.to_string());
}

fn part2(data: &Vec<i32>) {
    let mut increases = 0;
    let mut previous = 0;
    let mut start = 0;
    let mut slide = 3;

    for _current in data {
        if slide <= data.iter().count() {
            let slides: Vec<i32> = data[start..slide].to_vec();
            let slide_sum = slides.iter().sum();

            if slide_sum > previous && previous > 0 {
                increases = increases + 1;
            }
            previous = slide_sum;
            start = start + 1;
            slide = slide + 1;
        } 
    }

    println!("part 2 = {}", increases.to_string());

}

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
