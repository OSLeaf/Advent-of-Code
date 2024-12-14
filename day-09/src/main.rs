
fn main() {
    let input = include_str!("../input09.txt");
    let input_vec = input.chars().map(|x| x as u64 - 0x30).collect::<Vec<u64>>();

    println!("Part 1: {}", part_1(&input_vec));
    println!("Part 2: {}", part_2(input_vec));

}


fn part_1(input_vec: &[u64]) -> u64 {
    let converted_vector = convert_vector(input_vec);
    return checksum(&converted_vector);
}

fn part_2(input_vec: Vec<u64>) -> u64 {
    let memory_vector = create_memory_vector(&input_vec);
    let converted_vector = convert_memory_vector(input_vec, memory_vector);
    return checksum(&converted_vector);
}


fn convert_vector(input_vec: &[u64]) -> Vec<u64> {
    let mut pointer_1 = 0;
    let mut low_number = 0;

    let mut pointer_2 = input_vec.len() - 1;
    let mut spend_highnumbers = 0;
    let mut high_number = (input_vec.len() / 2) as u64;

    let mut converted_vec: Vec<u64> = Vec::new();
    let mut empty_space = false;

    while pointer_1 < pointer_2 {
        if !empty_space {
            for _ in 0..input_vec[pointer_1] {
                converted_vec.push(low_number);
            }
            pointer_1 += 1;
            low_number += 1;
            empty_space = true;
        } else {
            for _ in 0..input_vec[pointer_1] {
                if spend_highnumbers < input_vec[pointer_2] {
                    converted_vec.push(high_number);
                    spend_highnumbers += 1;
                } else {
                    pointer_2 -= 2;
                    high_number -= 1;
                    
                    converted_vec.push(high_number);
                    spend_highnumbers = 1;
                }

            }
            pointer_1 += 1;
            empty_space = false;
        }
    }
    for _ in 0..(input_vec[pointer_1] - spend_highnumbers) {
        converted_vec.push(low_number);
    }

    return converted_vec;
}

fn create_memory_vector(input_vec: &[u64]) -> Vec<u64> {
    let mut memory_vector: Vec<u64> = Vec::new();
    let mut empty_space = false;
    let mut number = 0;
    for i in input_vec {
        if !empty_space {
            for _ in 0..*i {
                memory_vector.push(number);
            }
            number += 1;
            empty_space = true;
        }
        else {
            for _ in 0..*i {
                memory_vector.push(0);
            }
            empty_space = false;
        }
    }
    return memory_vector;
}

fn convert_memory_vector(mut input_vec: Vec<u64>, mut memory_vector: Vec<u64>) -> Vec<u64> {
    let mut pointer_1 = input_vec.len() - 1;
    let mut pointer_2 = 1;
    let mut already_moved: Vec<usize> = Vec::new();

    let mut high_number = (input_vec.len() / 2) as u64;

    while pointer_1 > 1 {

        if already_moved.contains(&pointer_1) { //We are not allowed to move anything twice!
            pointer_1 -= 2;
            continue;
        }

        while pointer_2 < pointer_1 {
            if input_vec[pointer_2] >= input_vec[pointer_1] {
                let mut memory_pointer_1 = find_memory_location(&input_vec, &pointer_1);
                let mut memory_pointer_2 = find_memory_location(&input_vec, &pointer_2);
    
                for _ in 0..input_vec[pointer_1] {
                    memory_vector[memory_pointer_2] = high_number;
                    memory_pointer_2 += 1;
                }

                for _ in 0..input_vec[pointer_1] {
                    memory_vector[memory_pointer_1] = 0;
                    memory_pointer_1 += 1;
                }
    
                input_vec[pointer_2] -= input_vec[pointer_1];
                input_vec.insert(pointer_2, input_vec[pointer_1]);
                input_vec.insert(pointer_2, 0);
                already_moved.push(pointer_2 + 1);
                already_moved = already_moved.clone().into_iter().map(|x| if x > pointer_2 + 1 {x + 2} else {x}).collect::<Vec<usize>>();

                pointer_1 += 2;

                break;
            }
            pointer_2 += 2;

        }

        high_number -= 1;
        pointer_1 -= 2; 
        pointer_2 = 1;
    }
    return memory_vector;

}

fn find_memory_location(input_vec: &[u64], pointer_2: &usize) -> usize {
    let mut memory_pointer: usize = 0; 

    for p in 0..*pointer_2 {
        memory_pointer += input_vec[p] as usize;
    }
    return memory_pointer;
}

fn checksum(vector: &[u64]) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..vector.len() as u64 {
        sum += i * vector[i as usize]
    }
    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "2333133121414131402";
        let input_vec = input.chars().map(|x| x as u64 - 0x30).collect::<Vec<u64>>();

        assert_eq!(part_1(&input_vec), 1928);
    }

    #[test]
    fn test_part_2() {
        let input = "2333133121414131402";
        let input_vec = input.chars().map(|x| x as u64 - 0x30).collect::<Vec<u64>>();

        assert_eq!(part_2(input_vec), 2858);
    }
}