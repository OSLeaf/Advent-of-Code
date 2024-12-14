use std::collections::HashSet;

fn main() {
    let input = include_str!("../input10.txt");
    let mut trail_map: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let height_vec: Vec<u32> = line.chars().map(|x| x as u32 - 0x30).collect::<Vec<u32>>();
        trail_map.push(height_vec);
    }

    println!("Part 1: {}", part_1(&trail_map));
    println!("Part 2: {}", part_2(&trail_map));

}

fn part_1(trail_map: &[Vec<u32>]) -> u32 {
    let y_max = trail_map.len() -1;
    let x_max = trail_map[0].len() -1;
    let mut sum = 0;

    for y in 0..(y_max + 1) { //y_max + 1 bacause range does not take the last number
        for x in 0..(x_max + 1) { //x_max + 1 bacause range does not take the last number
            if trail_map[y][x] == 0 {
                sum += calculate_score(&trail_map, x, y);
            }
        }
    } 

    return sum;
}

fn part_2(trail_map: &[Vec<u32>]) -> u32 {
    let y_max = trail_map.len() -1;
    let x_max = trail_map[0].len() -1;
    let mut sum = 0;

    for y in 0..(y_max + 1) { //y_max + 1 bacause range does not take the last number
        for x in 0..(x_max + 1) { //x_max + 1 bacause range does not take the last number
            if trail_map[y][x] == 0 {
                sum += calculate_many_trails_score(&trail_map, x, y);
            }
        }
    } 

    return sum;
}

fn calculate_score(trail_map: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let peaks = reachable_peaks(trail_map, x, y);
    return peaks.len() as u32;
}

fn reachable_peaks(trail_map: &[Vec<u32>], x: usize, y: usize) -> HashSet<(usize, usize)> {
    let y_max = trail_map.len() - 1;
    let x_max = trail_map[0].len() - 1;
    let height = trail_map[y][x];
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();

    if height == 9 {
        peaks.insert((y, x));
        return peaks;
    } else {
        if y < y_max {if height + 1 == trail_map[y + 1][x] {peaks.extend(reachable_peaks(&trail_map, x, y + 1));}}
        if y > 0 {if height + 1 == trail_map[y - 1][x] {peaks.extend(reachable_peaks(&trail_map, x, y - 1));}}
        if x < x_max {if height + 1 == trail_map[y][x + 1] {peaks.extend(reachable_peaks(&trail_map, x + 1, y));}}
        if x > 0 {if height + 1 == trail_map[y][x - 1] {peaks.extend(reachable_peaks(&trail_map, x - 1, y));}}
        return peaks;
    }
}

fn calculate_many_trails_score(trail_map: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let y_max = trail_map.len() - 1;
    let x_max = trail_map[0].len() - 1;
    let height = trail_map[y][x];

    if height == 9 {
        return 1;
    } else {
        let mut sum = 0;
        if y < y_max {if height + 1 == trail_map[y + 1][x] {sum += calculate_many_trails_score(&trail_map, x, y + 1);}}
        if y > 0 {if height + 1 == trail_map[y - 1][x] {sum += calculate_many_trails_score(&trail_map, x, y - 1);}}
        if x < x_max {if height + 1 == trail_map[y][x + 1] {sum += calculate_many_trails_score(&trail_map, x + 1, y);}}
        if x > 0 {if height + 1 == trail_map[y][x - 1] {sum += calculate_many_trails_score(&trail_map, x - 1, y);}}
        return sum;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            vec![8,9,0,1,0,1,2,3],
            vec![7,8,1,2,1,8,7,4],
            vec![8,7,4,3,0,9,6,5],
            vec![9,6,5,4,9,8,7,4],
            vec![4,5,6,7,8,9,0,3],
            vec![3,2,0,1,9,0,1,2],
            vec![0,1,3,2,9,8,0,1],
            vec![1,0,4,5,6,7,3,2]
        ];
        assert_eq!(part_1(&input), 36);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            vec![8,9,0,1,0,1,2,3],
            vec![7,8,1,2,1,8,7,4],
            vec![8,7,4,3,0,9,6,5],
            vec![9,6,5,4,9,8,7,4],
            vec![4,5,6,7,8,9,0,3],
            vec![3,2,0,1,9,0,1,2],
            vec![0,1,3,2,9,8,0,1],
            vec![1,0,4,5,6,7,3,2]
        ];
        assert_eq!(part_2(&input), 81);
    }

}