fn main() {
    let input = include_str!("../input02.txt");
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let mut parset: Vec<i32> = Vec::new();
        for i in split {
            parset.push(i.parse::<i32>().unwrap());
        }
        reports.push(parset);
    }
    println!("Part 1: {}", part_1(&reports));
    println!("Part 2: {}", part_2(&reports));
}

fn part_1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for report in reports {
        if is_safe(&report) {
            safe += 1;
        }
    }
    return safe;
}

fn part_2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut r = report.clone();
            _ = r.remove(i);
            if is_safe(&r) {
                safe += 1;
                break;
            }
        }
    }
    return safe;
}

fn is_safe(report: &Vec<i32>) -> bool {
    return check_increasing(&report) || check_decreasing(&report)
}

fn check_increasing(report: &Vec<i32>) -> bool {
    let mut number = report[0];
    for i in 1..report.len() {
        if report[i] > number && number + 3 >= report[i] {
            number = report[i];
        } else {
            return false;
        }
    }
    return true;
}

fn check_decreasing(report: &Vec<i32>) -> bool {
    let mut number = report[0];
    for i in 1..report.len() {
        if  number > report[i] && number - 3 <= report[i] {
            number = report[i];
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let reports: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
          ];
        assert_eq!(part_1(&reports), 2);
    }

    #[test]
    fn test_part_2() {
        let reports: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]
          ];
        assert_eq!(part_2(&reports), 4);
    }

}