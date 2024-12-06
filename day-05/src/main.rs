use std::collections::HashMap;

fn main() {
    let rules = include_str!("../input05r.txt");
    let cases = include_str!("../input05c.txt");
    let mut rows: Vec<Vec<i32>> = Vec::new();
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in cases.lines() {
        let split = line.split(",").collect::<Vec<_>>();
        let mut parset: Vec<i32> = Vec::new();
        for i in split {
            parset.push(i.parse::<i32>().unwrap());
        }
        rows.push(parset);
    }
    for line in rules.lines() {
        let mut a = line.split("|");
        let b = a.next();
        let c = a.next();
        rule_map.entry(c.unwrap().parse::<i32>().unwrap()).or_default().push(b.unwrap().parse::<i32>().unwrap());
    }
    println!("Part 1: {}", part_1(&rows, &rule_map));
    println!("Part 2: {}", part_2(&rows, &rule_map));
}

fn part_1(rows: &Vec<Vec<i32>>, rule_map: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in rows {
        if check_row(row, &rule_map) {
            sum += row[row.len() / 2]
        }
    }
    return sum;
}

fn part_2(rows: &Vec<Vec<i32>>, rule_map: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut sum = 0;
    let mut counter = 0;
    for row in rows {
        println!("{}", counter);
        println!("Sum: {}", sum);
        counter += 1;
        if !check_row(row, &rule_map) {
            let mut contruct: Vec<i32> = Vec::new();
            for index in 0..row.len() {
                let mut location = contruct.len();
                if !contruct.contains(&row[index]) {
                    if rule_map.contains_key(&row[index]){    
                        for rule in &rule_map[&row[index]] {
                            if contruct.contains(rule) {
                                let new_location = contruct.iter().position(|&r| r == *rule).unwrap();
                                if new_location < location {
                                    location = new_location;
                                }
                            }
                        }
                    }
                }
                contruct.insert(location, row[index]);
            }
            sum += contruct[contruct.len() / 2];
        }
    }
    return sum;
}

fn check_row(row: &Vec<i32>, rule_map: &HashMap<i32, Vec<i32>>) -> bool {
    let mut past: Vec<i32> = Vec::new();
    for index in 0..row.len() {
        past.push(row[index]);

        if rule_map.contains_key(&row[index]) {
            for rule in &rule_map[&row[index]] {
                if row.contains(&rule) && !past.contains(&rule) {
                    return false;
                }
            }  
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_1() {
        let rules = vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13"
        ];
        let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for line in rules {
            let mut a = line.split("|");
            let b = a.next();
            let c = a.next();
            rule_map.entry(c.unwrap().parse::<i32>().unwrap()).or_default().push(b.unwrap().parse::<i32>().unwrap());
        }
        let cases = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47]
        ];
        //assert_eq!(part_1(&cases, &rule_map), 143);
        assert_eq!(part_2(&cases, &rule_map), 123);
        
    }
}
