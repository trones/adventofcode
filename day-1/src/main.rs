use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut increases = 0;
    let mut previous = 0;
    if let Ok(lines) = read("./src/input.txt") {
        for line in lines {
            if let Ok(data) = line {
                let current = data.parse::<i32>().unwrap();
                
                if current > previous && previous > 0 {
                    increases = increases + 1;
                }
                previous = data.parse::<i32>().unwrap();
            }
        }
        println!("{}", increases.to_string());
    }
}

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
