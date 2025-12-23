use std::io::{self, BufRead};
use std::time::Instant; // Import the timer module

fn part1(pairs: &Vec<(u64,u64)>){
    let mut count: u64 = 0; 
    for (a,b) in pairs{
        for i in *a..=*b{
            let length = (i as f64).log10() as usize + 1;
            if length % 2 != 0{
                continue;
            }
            let factor = 10_u64.pow((length as u32) / 2);
            if i/factor == i % factor{
                count += i;
            }
        }
    }
    println!("part1 = {}", count);
}

fn part2(pairs: &Vec<(u64, u64)>) {
    let mut count: u64 = 0;
    
    for (a, b) in pairs {
        for i in *a..=*b {
            if i < 10 { continue; }
            
            let length = i.ilog10() + 1;

            for sub_len in 1..=(length / 2) {
                if length % sub_len != 0 { continue; }

                let divisor = 10_u64.pow(sub_len);
                let pattern = i % divisor;
                
                let mut temp = i;
                let mut valid = true;
                
                for _ in 0..(length / sub_len) {
                    if temp % divisor != pattern {
                        valid = false;
                        break;
                    }
                    temp /= divisor;
                }
                
                if valid {
                    count += i;
                    break;
                }
            }
        }
    }
    println!("part2 = {}", count);
}

fn main() {
    let stdin = io::stdin();
    let lines: String = stdin.lock().lines().map(|l| l.unwrap()).collect();
    
    // Parse inputs
    let pairs: Vec<(u64, u64)> = lines
        .split(',')
        .filter_map(|s| s.trim().split_once('-'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let start_p1 = Instant::now();
    part1(&pairs);
    let duration_p1 = start_p1.elapsed();
    println!("Part 1 time: {:.2?}", duration_p1); 

    let start_p2 = Instant::now();
    part2(&pairs);
    let duration_p2 = start_p2.elapsed();
    println!("Part 2 time: {:.2?}", duration_p2);
    //nicks time took: 181.555564ms
    //Part 1: 52316131093
    //took: 1.033159319s
    //Part 2: 69564213293

    //mine part1 = 52316131093
    //Part 1 time: 112.52ms
    //part2 = 69564213293
    //Part 2 time: 197.80ms
}