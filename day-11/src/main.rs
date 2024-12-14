use std::collections::HashMap;

fn main() {
    let input = include_str!("../input11.txt");
    let stone_vec: Vec<u128> = input.split_whitespace().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();

    let recursio_deph = 75;

    println!("Part 1: {}", part_1_2(stone_vec.clone(), recursio_deph));
}

fn part_1_2(stone_vec: Vec<u128>, recursion_deph: u128) -> u128 {
    let mut map = vector_to_hashmap(stone_vec);
    for _ in 0..recursion_deph {
        map = expand(map);
    }
    return map.values().sum::<u128>();
}

fn expand(map: HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut new_map: HashMap<u128, u128> = HashMap::new();

    for value in map.keys() {
        if *value == 0 {
            *new_map.entry(1).or_insert(0) += map[value];
        } 
        else if evennumber(*value) {
            let split_values = calc_split_values(*value);
            *new_map.entry(split_values.0).or_insert(0) += map[value];
            *new_map.entry(split_values.1).or_insert(0) += map[value];
        } 
        else {
            *new_map.entry(value * 2024).or_insert(0) += map[value];
        }
    }
    return new_map;
}


fn evennumber(number: u128) -> bool {
    return number.to_string().len() % 2 == 0;
}

fn calc_split_values(number: u128) -> (u128, u128) {
    let mut s_1 = number.to_string();
    let s_2 = s_1.split_off(s_1.len() / 2);
    return (s_1.parse::<u128>().unwrap(), s_2.parse::<u128>().unwrap());
}

fn vector_to_hashmap(vec: Vec<u128>) -> HashMap<u128, u128> {
    let mut map = HashMap::new();
    for value in vec {
        *map.entry(value).or_insert(0) += 1;
    }
    return map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let stone_vec = vec![125, 17];
        let mut map = vector_to_hashmap(stone_vec);
        for _ in 0..6 {
            map = expand(map);
        }
        assert_eq!(map.values().sum::<u128>(), 22);
    }

}