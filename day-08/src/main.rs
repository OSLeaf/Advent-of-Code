use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input08.txt");
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        let char_vec: Vec<_> = line.chars().collect();

        x = 0;
        for i in char_vec {
            if i != '.' {
                antennas.entry(i).or_default().push((x, y))
            }
            x += 1;
        }
        y += 1;
    }

    println!("Part 1: {}", part_1(&antennas, x, y));
    println!("Part 2: {}", part_2(&antennas, x, y));

}

fn part_1(antennas: &HashMap<char, Vec<(i32, i32)>>, x_max: i32, y_max: i32) -> usize  {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for index in antennas.keys() {
        let antenna_vector = &antennas[index];
        for i in antenna_vector {
            for j in antenna_vector {
                if i != j {
                    let x_difference = i.0 - j.0;
                    let y_difference = i.1 - j.1;
                    if i.0 + x_difference >= 0 && i.0 + x_difference < x_max && i.1 + y_difference >= 0 && i.1 + y_difference < y_max { // Check if inside the map
                        antinodes.insert((i.0 + x_difference, i.1 + y_difference));
                    }
                    if i.0 - 2 * x_difference >= 0 && i.0 - 2 * x_difference < x_max && i.1 - 2 * y_difference >= 0 && i.1 - 2 * y_difference < y_max { // Check if inside the map
                        antinodes.insert((i.0 - 2 * x_difference, i.1 - 2 * y_difference));
                    }
                }
            }
        }
    }
    return antinodes.len();
}

fn part_2(antennas: &HashMap<char, Vec<(i32, i32)>>, max_x: i32, max_y: i32) -> usize  {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for index in antennas.keys() {
        let antenna_vector = &antennas[index];
        for i in antenna_vector {
            for j in antenna_vector {
                if i != j {
                    let x_difference = i.0 - j.0;
                    let y_difference = i.1 - j.1;
                    let mut current_x = i.0;
                    let mut current_y = i.1;
                    while current_x >= 0 && current_x < max_x && current_y >= 0 && current_y < max_y {
                        antinodes.insert((current_x, current_y));
                        current_x += x_difference;
                        current_y += y_difference;
                    }
                    current_x = i.0;
                    current_y = i.1;
                    while current_x >= 0 && current_x < max_x && current_y >= 0 && current_y < max_y {
                        antinodes.insert((current_x, current_y));
                        current_x -= x_difference;
                        current_y -= y_difference;
                    }
                }
            }
        }
    }
    return antinodes.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let antenna_vec: Vec<(char, i32, i32)> = vec![('O', 1, 8), ('O', 2, 5), ('O', 3, 7), ('O', 4, 4), ('A', 5, 6), ('A', 8, 8), ('A', 9, 9)];
        for i in antenna_vec {
            antennas.entry(i.0).or_default().push((i.1, i.2))
        }
        assert_eq!(part_1(&antennas, 12, 12), 14);
    }
    #[test]
    fn test_part_2() {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let antenna_vec: Vec<(char, i32, i32)> = vec![('O', 1, 8), ('O', 2, 5), ('O', 3, 7), ('O', 4, 4), ('A', 5, 6), ('A', 8, 8), ('A', 9, 9)];
        for i in antenna_vec {
            antennas.entry(i.0).or_default().push((i.1, i.2))
        }
        assert_eq!(part_2(&antennas, 12, 12), 34);
    }
}