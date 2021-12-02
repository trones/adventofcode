
fn main() {
    let input = include_str!("../input");
    let course: Vec<String> = input
        .split('\n')
        .map(|l| l.trim().parse().expect("Invalid string"))
        .collect();

    part1(&course);
    part2(&course);

}

fn part2(course: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for c in course {

        let (dir, units) = c.split_once(' ').unwrap();
        let amount: i32 = units.parse().unwrap();
        
        match dir {
            "forward" => {
                horizontal = horizontal + amount;
                depth = depth + (aim * amount);
            },
            "down" => {
                aim = aim + amount;
            },
            "up" => {
                aim = aim - amount;
            }
            _ => {}
        }

        
    }
    let result = horizontal * depth;
    println!("Result 2 => {}", result.to_string());

}

fn part1(course: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;
    
    for c in course {

        let (dir, units) = c.split_once(' ').unwrap();
        let amount: i32 = units.parse().unwrap();
        
        match dir {
            "forward" => {
                horizontal = horizontal + amount;
            },
            "down" => {
                depth = depth + amount;
            },
            "up" => {
                depth = depth - amount;
            }
            _ => {}
        }

        
    }
    let result = horizontal * depth;
    println!("Result => {}", result.to_string());
}
