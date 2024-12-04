fn main() {
    let input = include_str!("../input04.txt");
    let mut rows: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        rows.push(line.chars().collect());
    }
    println!("Part 1: {}", part_1(&rows));
    println!("Part 2: {}", part_2(&rows));
}

fn part_1(rows: &Vec<Vec<char>>) -> i32 {
    let mut amount = 0;
    for x in 0..rows.len() {
        for y in 0..rows[0].len() {
            if rows[x][y] == 'X'{
                amount += star_search(rows, x as i32, y as i32);
            }
        }
    }
    return amount;
}

fn part_2(rows: &Vec<Vec<char>>) -> i32 {
    let mut amount = 0;
    for x in 0..rows.len() {
        for y in 0..rows[0].len() {
            if rows[x][y] == 'A'{
                amount += x_search(rows, x as i32, y as i32);
            }
        }
    }
    return amount;
}

fn star_search(rows: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let directions = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1], vec![1, 1], vec![1, -1], vec![-1, -1], vec![-1, 1]];
    let mut amount = 0;
    for i in directions {
        if (x + 3 * i[0]) >= 0 && (x + 3 * i[0]) < rows.len() as i32 && (y + 3 * i[1]) >= 0 && (y + 3 * i[1]) < rows[0].len() as i32 {
            if rows[(x + i[0]) as usize][(y + i[1]) as usize] == 'M' && rows[(x + 2 * i[0]) as usize][(y + 2* i[1]) as usize] == 'A' && rows[(x + 3* i[0]) as usize][(y + 3 * i[1]) as usize] == 'S' {
                amount += 1;
            }
        }
    }
    return amount;
}

fn x_search(rows: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut trigger1 = false;
    let mut trigger2 = false;
    if x > 0 && x < rows.len() as i32 -1 && y > 0 && y < rows[0].len() as i32 -1  {
        match rows[(x - 1) as usize][(y - 1) as usize] {
            'M' => if rows[(x + 1) as usize][(y + 1) as usize] == 'S'{trigger1 = true;},
            'S' => if rows[(x + 1) as usize][(y + 1) as usize] == 'M'{trigger1 = true;},
            _ => ()
        }
        match rows[(x + 1) as usize][(y - 1) as usize] {
            'M' => if rows[(x - 1) as usize][(y + 1) as usize] == 'S'{trigger2 = true;},
            'S' => if rows[(x - 1) as usize][(y + 1) as usize] == 'M'{trigger2 = true;},
            _ => ()
        }
    }
    if trigger1 && trigger2 {
        return 1;
    }
    return 0;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let rows: Vec<Vec<char>> = vec![
            vec!['M','M','M','S','X','X','M','A','S','M'],
            vec!['M','S','A','M','X','M','S','M','S','A'],
            vec!['A','M','X','S','X','M','A','A','M','M'],
            vec!['M','S','A','M','A','S','M','S','M','X'],
            vec!['X','M','A','S','A','M','X','A','M','M'],
            vec!['X','X','A','M','M','X','X','A','M','A'],
            vec!['S','M','S','M','S','A','S','X','S','S'],
            vec!['S','A','X','A','M','A','S','A','A','A'],
            vec!['M','A','M','M','M','X','M','M','M','M'],
            vec!['M','X','M','X','A','X','M','A','S','X']
          ];
        assert_eq!(part_1(&rows), 18);
    }
    #[test]
    fn test_part_2() {
        let rows: Vec<Vec<char>> = vec![
            vec!['M','M','M','S','X','X','M','A','S','M'],
            vec!['M','S','A','M','X','M','S','M','S','A'],
            vec!['A','M','X','S','X','M','A','A','M','M'],
            vec!['M','S','A','M','A','S','M','S','M','X'],
            vec!['X','M','A','S','A','M','X','A','M','M'],
            vec!['X','X','A','M','M','X','X','A','M','A'],
            vec!['S','M','S','M','S','A','S','X','S','S'],
            vec!['S','A','X','A','M','A','S','A','A','A'],
            vec!['M','A','M','M','M','X','M','M','M','M'],
            vec!['M','X','M','X','A','X','M','A','S','X']
          ];
        assert_eq!(part_2(&rows), 9);
    }
}
