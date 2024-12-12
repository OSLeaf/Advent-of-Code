
fn main() {
    let input = include_str!("../input07.txt");
    let mut answers: Vec<u64> = Vec::new();
    let mut values: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        let mut split = line.split(':');
        answers.push(split.next().unwrap().parse::<u64>().unwrap());
        let value_row = split.next().unwrap().split_whitespace();
        let mut value_vec: Vec<u64> = Vec::new();
        for i in value_row {
            value_vec.push(i.parse::<u64>().unwrap());
        }
        values.push(value_vec);
    }
    println!("Part 1: {}", part_1_and_2(&answers, &values, false));
    println!("Part 2: {}", part_1_and_2(&answers, &values, true));
}

fn part_1_and_2(answers: &[u64], values: &[Vec<u64>], conc_enabled: bool) -> u64 {
    let mut sum = 0;
    let zap = answers.into_iter().zip(values.into_iter());
    for line in zap {
        if possible(&line.0, &line.1, line.1[0], 1, conc_enabled) {
            sum += line.0;
        }
    }
    return sum;
}

fn possible(answer: &u64, value_vec: &[u64], current: u64, index: usize, conc_enabled: bool) -> bool {
    if index < value_vec.len() {   //Check that we have not run out of values. If we have return false.
        let next_value = value_vec[index];
        let add = possible(&answer, &value_vec, current + next_value, index + 1, conc_enabled);
        let mult = possible(&answer, &value_vec, current * next_value, index + 1, conc_enabled);
        let mut conc = false; //Used only in part 2
        if conc_enabled {
            conc = possible(&answer, &value_vec, format!("{current}{next_value}").parse::<u64>().unwrap(), index + 1, conc_enabled);
        }

        return add || mult || conc;
    } else {
        if current == *answer {
            return true;
        } else{
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let answers: Vec<u64> = vec![190, 3267, 83, 156, 7290, 161011, 192, 21037, 292];
        let values: Vec<Vec<u64>> = vec![
            vec![10, 19],
            vec![81, 40, 27],
            vec![17, 5],
            vec![15, 6],
            vec![6, 8, 6, 15],
            vec![16, 10, 13],
            vec![17, 8, 14],
            vec![9, 7, 18, 13],
            vec![11, 6, 16, 20]
          ];

        assert_eq!(part_1_and_2(&answers, &values, false), 3749);
    }

    #[test]
    fn test_part_2() {
        let answers: Vec<u64> = vec![190, 3267, 83, 156, 7290, 161011, 192, 21037, 292];
        let values: Vec<Vec<u64>> = vec![
            vec![10, 19],
            vec![81, 40, 27],
            vec![17, 5],
            vec![15, 6],
            vec![6, 8, 6, 15],
            vec![16, 10, 13],
            vec![17, 8, 14],
            vec![9, 7, 18, 13],
            vec![11, 6, 16, 20]
          ];

        assert_eq!(part_1_and_2(&answers, &values, true), 11387);
    }
}