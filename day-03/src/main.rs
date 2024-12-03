use std::fs;
use regex::Regex;

fn main() {
    let path = "input03.txt".to_string();
    let input = fs::read_to_string(path).expect("No input file found!");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &String) -> i32 {
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(&input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }
    return sum;
}

fn part_2(input: &String) -> i32 {
    let re = Regex::new(r"(mul\((-?\d+),(-?\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut sum = 0;
    let mut active = true;

    for cap in re.captures_iter(&input) {
        if let Some(found) = cap.get(0) {
            if found.as_str().starts_with("mul") && active {
                let x: i32 = cap[2].parse().unwrap();
                let y: i32 = cap[3].parse().unwrap();
                sum += x * y;
            } else if found.as_str() == "do()" {
                active = true;
            } else if found.as_str() == "don't()" {
                active = false;
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: String = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(part_1(&input), 161);
    }

    #[test]
    fn test_part_2() {
        let input: String = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(part_2(&input), 48);
    }
}