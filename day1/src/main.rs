use std::io::{self, BufRead};
use std::time::Instant; // 1. Add this import

const MAX_SIZE: i32 = 100;

fn left(current_index: i32, steps: i32) -> i32 {
    (current_index - steps).rem_euclid(MAX_SIZE)
}

fn right(current_index: i32, steps: i32) -> i32 {
    (current_index + steps).rem_euclid(MAX_SIZE)
}

fn count_zeros(current: i32, steps: i32, is_right: bool) -> i32 {
    if is_right {
        return (current + steps) / MAX_SIZE;
    } else {
        let dist_to_zero = if current == 0 { MAX_SIZE } else { current };
        if steps < dist_to_zero {
            return 0; 
        } else {
            return 1 + (steps - dist_to_zero) / MAX_SIZE;
        }
    }
}

fn part1(lines: &Vec<String>) {
    let mut my_list = vec![0; MAX_SIZE as usize];
    let mut current_index = 50;

    for text in lines {
        let command = text.chars().next().unwrap();
        let value: i32 = text[1..].parse().unwrap();

        if command == 'R' {
            current_index = right(current_index, value);
        } else {
            current_index = left(current_index, value);
        }
        my_list[current_index as usize] += 1;
    }
    
    my_list.sort();
    println!("Part 1 Answer (Max Value): {}", my_list[(MAX_SIZE - 1) as usize]);
}

fn part2(lines: &Vec<String>) {
    let mut current_index = 50;
    let mut total_zeros = 0;

    for text in lines {
        let command = text.chars().next().unwrap();
        let value: i32 = text[1..].parse().unwrap();

        if command == 'R' {
            total_zeros += count_zeros(current_index, value, true);
            current_index = right(current_index, value);
        } else {
            total_zeros += count_zeros(current_index, value, false);
            current_index = left(current_index, value);
        }
    }

    println!("Part 2 Answer (Total Zeros): {}", total_zeros);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let start_p1 = Instant::now();
    part1(&lines);
    let duration_p1 = start_p1.elapsed();
    println!("Part 1 took: {:?}", duration_p1);

    let start_p2 = Instant::now();
    part2(&lines);
    let duration_p2 = start_p2.elapsed();
    println!("Part 2 took: {:?}", duration_p2);
}