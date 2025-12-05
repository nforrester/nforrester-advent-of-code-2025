use std::fs;

fn parse_range(s: &str) -> (i64, i64) {
    let num_strings: Vec<_> = s.split("-").collect();
    assert!(num_strings.len() == 2);
    let lo = num_strings[0].parse().unwrap();
    let hi = num_strings[1].parse().unwrap();
    (lo, hi)
}

fn count_in_range(ranges: &Vec<(i64, i64)>, ids: Vec<i64>) -> usize {
    ids.iter().filter(|id| ranges.iter().any(|(a, b)| a <= *id && *id <= b)).count()
}

fn trim_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort();

    let mut result = Vec::<(i64, i64)>::new();

    for (new_a, new_b) in ranges {
        if let Some((_, old_b)) = result.last() {
            if *old_b < new_b {
                if *old_b < new_a {
                    result.push((new_a, new_b));
                } else {
                    result.push((old_b + 1, new_b));
                }
            }
        } else {
            result.push((new_a, new_b));
        }
    }
    result
}

fn all_fresh(mut ranges: Vec<(i64, i64)>) -> i64 {
    ranges = trim_ranges(ranges);
    let mut result = 0;
    for (a, b) in ranges {
        result += b - a + 1;
    }
    result
}

fn main() {
    let input: String = fs::read_to_string("dat/d5.txt").unwrap();
    let input_halves: Vec<_> = input.split("\n\n").collect();
    assert!(input_halves.len() == 2);
    let ranges: Vec<_> = input_halves[0].lines().map(|l| parse_range(l)).collect();
    let ids: Vec<i64> = input_halves[1].lines().map(|l| l.parse().unwrap()).collect();

    println!("Part 1: {}", count_in_range(&ranges, ids));
    println!("Part 2: {}", all_fresh(ranges));
}
