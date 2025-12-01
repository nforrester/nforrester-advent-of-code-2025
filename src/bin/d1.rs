use std::fs;

const DIAL_SIZE: i64 = 100;
const START_POS: i64 = 50;

fn decode_movement(line: &str) -> i64 {
    let ticks: i64 = line[1..].parse().unwrap();
    match line.chars().nth(0) {
        Some('L') => -ticks,
        Some('R') => ticks,
        _ => panic!(),
    }
}

fn count_zero_ends((pos, zero_ends): (i64, i64), twist: i64) -> (i64, i64) {
    let new_pos = (pos + twist).rem_euclid(DIAL_SIZE);
    (
        new_pos,
        zero_ends + (if new_pos == 0 { 1 } else { 0 }),
    )
}

fn count_zero_clicks((pos, zero_clicks): (i64, i64), twist: i64) -> (i64, i64) {
    let twisted = pos + twist;
    (
        twisted.rem_euclid(DIAL_SIZE),
        (
            zero_clicks +
            (twisted / DIAL_SIZE).abs() +
            (if twisted <= 0 && pos > 0 { 1 } else { 0 })
        ),
    )
}

fn main() {
    let input = fs::read_to_string("dat/d1.txt").unwrap();
    let moves: Vec<_> = input.lines().map(|l| decode_movement(l)).collect();

    let (_, zero_ends) = moves.iter().fold((START_POS, 0), |p, m| count_zero_ends(p, *m));
    println!("Part 1: {}", zero_ends);

    let (_, zero_clicks) = moves.iter().fold((START_POS, 0), |p, m| count_zero_clicks(p, *m));
    println!("Part 2: {}", zero_clicks);
}
