use std::fs;
use std::iter::zip;

fn analyze_splits(mut beams: Vec<bool>, mut splits: i64, line: &str) -> (Vec<bool>, i64) {
    if beams.len() == 0 {
        return (line.chars().map(|c| c == 'S').collect(), 0);
    }
    let hits: Vec<bool> = zip(line.chars(), beams.iter()).map(|(c, b)| c == '^' && *b).collect();
    for (i, hit) in zip(0.., &hits) {
        if *hit {
            beams[i] = false;
            splits += 1;
        }
    }
    for (i, hit) in zip(0.., hits) {
        if hit {
            if i > 0 {
                beams[i-1] = true;
            }
            if i < beams.len() - 1 {
                beams[i+1] = true;
            }
        }
    }
    return (beams, splits);
}

fn analyze_timelines(beams: Vec<i64>, line: &str) -> Vec<i64> {
    if beams.len() == 0 {
        return line.chars().map(|c| match c { 'S' => 1, _ => 0 }).collect();
    }
    let hits: Vec<i64> = zip(line.chars(), beams.iter()).map(|(c, b)| match c { '^' => *b, _ => 0 }).collect();
    let mut new_beams = vec![0; beams.len()];
    for (i, c) in zip(0.., line.chars()) {
        if c != '^' {
            new_beams[i] += beams[i];
        }
    }
    for (i, n_hit) in zip(0.., &hits) {
        if i > 0 {
            new_beams[i-1] += n_hit;
        }
        if i < new_beams.len() - 1 {
            new_beams[i+1] += n_hit;
        }
    }
    return new_beams;
}

fn main() {
    let input = fs::read_to_string("dat/d7.txt").unwrap();
    let (_, splits) = input.lines().fold((vec![], 0), |(beams, splits), line| analyze_splits(beams, splits, line));
    println!("Part 1: {}", splits);
    let timelines: i64 = input.lines().fold(vec![], |beams, line| analyze_timelines(beams, line)).iter().sum();
    println!("Part 2: {}", timelines);
}
