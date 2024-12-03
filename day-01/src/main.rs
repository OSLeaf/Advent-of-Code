use itertools::Itertools;
fn main() {
    let input = include_str!("../input01.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i32>().unwrap());
        right.push(split.next().unwrap().parse::<i32>().unwrap());
    }
    println!("Part 1: {}", part_1(&mut left, &mut right));
    println!("Part 1: {}", part_2(&mut left, &mut right));
}


fn part_1(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs()
    }
    return sum;
}
fn part_2(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    let counts = right.into_iter().counts();
    let mut similarity = 0;
    for number in left {
        let count = counts.get(number);
        match count { //if count is 0, hashmap gives None not i32
            None => (), //"similarity += left[i] * 0"
            _ => similarity += *number * *count.unwrap() as i32,
        }
    }
    return similarity;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_1(&mut left, &mut right), 11);
    }

    #[test]
    fn test_part_2() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_2(&mut left, &mut right), 31);
    }
}