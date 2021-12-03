fn main() {
    let input = include_str!("../input_prod");
    let diagnostic: Vec<String> = input
        .split('\n')
        .map(|l| l.trim().parse().expect("Invalid string"))
        .collect();

   part1(&diagnostic);
   println!("Part 2: {}", part2(&diagnostic, true) * part2(&diagnostic, false));
}

fn part2(diagnostic: &Vec<String>, val: bool) -> i32 {
    let mut idx = 0;
    let mut values = diagnostic.clone();
    while values.len() > 1 {
        // tar reda på hur många ettor det finns i varje kolumn
        let ones = values.iter().map(|c| c.chars().nth(idx).unwrap()).filter(|c| c.clone() == '1').count();
        
        if ones >= (values.len() - ones) {
            values.retain(|c|c.chars().nth(idx).unwrap() == if val {'1'} else {'0'})
        } else {
            values.retain(|c|c.chars().nth(idx).unwrap() == if val {'0'} else {'1'})
        }
        idx += 1
    }
    i32::from_str_radix(values[0].as_str(),2).unwrap()
}


fn part1(diagnostic: &Vec<String>) {
    let mut gammaRate = 0;
    let mut epsilonRate = 0;
    for pos in 0..diagnostic[0].len() {
        gammaRate <<=1;
        epsilonRate <<=1;
        // tar reda på hur många ettor det finns i varje kolumn
        let ones = diagnostic.iter().map(|c| c.chars().nth(pos).unwrap()).filter(|c| c.clone() == '1').count();

        // ones är större än zero
        if ones >= (diagnostic.len() - ones) {
            gammaRate += 1
        } else {
            epsilonRate += 1;
        }

    }
    println!("{}", gammaRate * epsilonRate);
}

