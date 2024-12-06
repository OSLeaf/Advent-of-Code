use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let input = include_str!("../input06.txt");
    let mut reports: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        reports.push(line.chars().collect());
    }
    let mut x_coord = 0;
    let mut y_coord = 0;
    let mut d: Direction = Direction::North;

    for x in 0..reports.len() {
        for y in 0..reports[x].len() {
            match reports[x][y] {
                '^' => {
                    x_coord = x;
                    y_coord = y;
                    d = Direction::North;
                    reports[x][y] = '.';
                    break; 
                }
                'v' => {
                    x_coord = x;
                    y_coord = y;
                    d = Direction::South;
                    reports[x][y] = '.';
                    break; 
                }
                '<' => {
                    x_coord = x;
                    y_coord = y;
                    d = Direction::West;
                    reports[x][y] = '.';
                    break; 
                }
                '>' => {
                    x_coord = x;
                    y_coord = y;
                    d = Direction::East;
                    reports[x][y] = '.';
                    break; 
                }
                _ => {();}
            }
        }
    }
    println!("Part 1: {}", part_1(&reports, x_coord.clone(), y_coord.clone(), d.clone()));
    println!("Part 2: {}", part_2(&mut reports, x_coord.clone(), y_coord.clone(), d.clone()));
}

fn part_1(reports: &Vec<Vec<char>>, mut x_coord: usize, mut y_coord: usize, mut d: Direction) -> i32 {
    let mut sum = 1;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        //println!("{:?}", visited);
        //println!("X: {} Y: {}", x_coord, y_coord);
        match d {
            Direction::South =>{
                if x_coord + 1 < reports.len() {
                    match reports[x_coord + 1][y_coord] {
                        '.' => {
                            if !visited.contains(&(x_coord, y_coord)) {sum += 1; visited.insert((x_coord, y_coord));};
                            x_coord += 1; 
                        }
                        '#' => {d = Direction::West;}
                        _ => {();}
                    }
                } else {return sum;}
            }
            Direction::East => {
                if y_coord + 1 < reports[0].len() {
                    match reports[x_coord][y_coord + 1] {
                        '.' => { 
                            if !visited.contains(&(x_coord, y_coord)) {sum += 1; visited.insert((x_coord, y_coord));};
                            y_coord += 1; 
                        }
                        '#' => {d = Direction::South;}
                        _ => {();}
                    }
                } else {return sum;}
            }
            Direction::West => {
                if y_coord != 0 {
                    match reports[x_coord][y_coord - 1] {
                        '.' => {
                            if !visited.contains(&(x_coord, y_coord)) {sum += 1; visited.insert((x_coord, y_coord));};
                            y_coord -= 1; 
                        }
                        '#' => {d = Direction::North;}
                        _ => {();}
                    }
                } else {return sum;}
            }
            Direction::North => {
                if x_coord != 0 {
                    match reports[x_coord - 1][y_coord] {
                        '.' => {
                            if !visited.contains(&(x_coord, y_coord)) {sum += 1; visited.insert((x_coord, y_coord));};
                            x_coord -= 1; 
                        }
                        '#' => {d = Direction::East;}
                        _ => {();}
                    }
                } else {return sum;}
            }
        }
    }
}

fn part_2(reports: &mut Vec<Vec<char>>, x_coord: usize, y_coord: usize, d: Direction) -> i32 {
    let mut sum = 0;
    for x in 0..reports.len() {
        for y in 0..reports.len() {
            if (x != x_coord || y != y_coord) && reports[x][y] == '.' {    
                reports[x][y] = 'O';
                if check_for_loop(reports, x_coord.clone(), y_coord.clone(), d) {sum += 1;}
                reports[x][y] = '.';}
        }
    }
    return sum;
}

fn check_for_loop(reports: &Vec<Vec<char>>, mut x_coord: usize, mut y_coord: usize, mut d: Direction) -> bool {
    let mut visited: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    loop {
        //println!("{:?}", visited);
        //println!("X: {} Y: {}", x_coord, y_coord);
        match d {
            Direction::South =>{
                if x_coord + 1 < reports.len() {
                    match reports[x_coord + 1][y_coord] {
                        '.' => {
                            if visited.contains(&((x_coord, y_coord),(x_coord + 1, y_coord))) {return true;}
                            visited.insert(((x_coord, y_coord),(x_coord + 1, y_coord)));
                            x_coord += 1; 
                        }
                        _ => {d = Direction::West;}
                    }
                } else {return false;}
            }
            Direction::East => {
                if y_coord + 1 < reports[0].len() {
                    match reports[x_coord][y_coord + 1] {
                        '.' => { 
                            if visited.contains(&((x_coord, y_coord),(x_coord, y_coord + 1))) {return true;}
                            visited.insert(((x_coord, y_coord),(x_coord, y_coord + 1)));
                            y_coord += 1; 
                        }
                        _ => {d = Direction::South;}
                    }
                } else {return false;}
            }
            Direction::West => {
                if y_coord != 0 {
                    match reports[x_coord][y_coord - 1] {
                        '.' => {
                            if visited.contains(&((x_coord, y_coord),(x_coord, y_coord - 1))) {return true;}
                            visited.insert(((x_coord, y_coord),(x_coord, y_coord - 1)));
                            y_coord -= 1; 
                        }
                        _ => {d = Direction::North;}
                    }
                } else {return false;}
            }
            Direction::North => {
                if x_coord != 0 {
                    match reports[x_coord - 1][y_coord] {
                        '.' => {
                            if visited.contains(&((x_coord, y_coord),(x_coord - 1, y_coord))) {return true;}
                            visited.insert(((x_coord, y_coord),(x_coord - 1, y_coord)));
                            x_coord -= 1; 
                        }
                        _ => {d = Direction::East;}
                    }
                } else {return false;}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let reports: Vec<Vec<char>> = vec![
            vec!['.','.','.','.','#','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','#'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','#','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','#','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['.','#','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','#','.'],
            vec!['#','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','#','.','.','.']
          ];
        assert_eq!(part_1(&reports, 6, 4, Direction::North), 41);
    }
    #[test]
    fn test_part_2() {
        let mut reports: Vec<Vec<char>> = vec![
            vec!['.','.','.','.','#','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','#'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','#','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','#','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['.','#','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','#','.'],
            vec!['#','.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','#','.','.','.']
          ];
        assert_eq!(part_2(&mut reports, 6, 4, Direction::North), 6);
    }
}