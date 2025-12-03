use std::fs;
use std::cmp::max;

fn parse_bank(s: &str) -> Vec<i64> {
    s.chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}

fn max_of_two(bank: &Vec<i64>) -> i64 {
    let mut m = 0;
    for i in 0..bank.len() {
        for j in i+1..bank.len() {
            m = max(m, bank[i] * 10 + bank[j]);
        }
    }
    m
}

fn max_of_twelve(bank: &Vec<i64>) -> i64 {
    let mut joltage = 0;
    let mut next = 0;
    for tail in (0..12).rev() {
        'hope_loop: for hope in (1..=9).rev() {
            for this in next..bank.len()-tail {
                if bank[this] == hope {
                    joltage *= 10;
                    joltage += bank[this];
                    next = this + 1;
                    break 'hope_loop;
                }
            }
        }
    }
    joltage
}

fn main() {
    let input: String = fs::read_to_string("dat/d3.txt").unwrap();
    let jolts_two: i64 = input.lines().map(parse_bank).map(|b| max_of_two(&b)).sum();
    println!("Part 1: {}", jolts_two);
    let jolts_twelve: i64 = input.lines().map(parse_bank).map(|b| max_of_twelve(&b)).sum();
    println!("Part 2: {}", jolts_twelve);
}
