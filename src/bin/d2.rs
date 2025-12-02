use std::fs;

fn parse_range(s: &str) -> std::ops::RangeInclusive<i64> {
    let num_strings: Vec<_> = s.split("-").collect();
    assert!(num_strings.len() == 2);
    let lo = num_strings[0].parse().unwrap();
    let hi = num_strings[1].parse().unwrap();
    lo..=hi
}

fn is_twice(id: i64) -> bool {
    let s = format!("{}", id);
    let a = &s[..(s.len()/2)];
    let b = &s[(s.len()/2)..];
    a == b
}

fn is_repeated(id: i64) -> bool {
    let s = format!("{}", id);
    for repeats in 2..=s.len() {
        let pattern = &s[..(s.len()/repeats)];
        if pattern.repeat(repeats) == s {
            return true;
        }
    }
    return false;
}

fn main() {
    let input: String = fs::read_to_string("dat/d2.txt").unwrap().split_whitespace().collect();
    let to_check = input.split(",").flat_map(|s| parse_range(s));
    let sum_twice: i64 = to_check.clone().filter(|i| is_twice(*i)).sum();
    println!("Part 1: {}", sum_twice);
    let sum_repeated: i64 = to_check.filter(|i| is_repeated(*i)).sum();
    println!("Part 2: {}", sum_repeated);
}
