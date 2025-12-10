use std::fs;
use std::cmp::{max, min};

fn part1(reds: &Vec<(i64, i64)>) -> i64 {
    let mut best = 0;
    for i in 0..reds.len()-1 {
        let (x1, y1) = reds[i];
        for j in i+1..reds.len() {
            let (x2, y2) = reds[j];
            let dx = (x1 - x2).abs() + 1;
            let dy = (y1 - y2).abs() + 1;
            let area = dx * dy;
            best = max(best, area);
        }
    }
    best
}

fn part2(reds: &Vec<(i64, i64)>) -> i64 {
    let mut best = 0;
    for i in 0..reds.len()-1 {
        let (x1, y1) = reds[i];
        'next_rect: for j in i+1..reds.len() {
            let (x2, y2) = reds[j];
            let dx = (x1 - x2).abs() + 1;
            let dy = (y1 - y2).abs() + 1;
            let area = dx * dy;

            if best > area {
                continue;
            }

            for p in 0..reds.len() {
                let q = (p+1) % reds.len();
                let (xp, yp) = reds[p];
                let (xq, yq) = reds[q];
                if xp == xq {
                    if min(x1, x2) < xp && xp < max(x1, x2) {
                        for w in min(yp, yq)..=max(yp, yq) {
                            if min(y1, y2) < w && w < max(y1, y2) {
                                continue 'next_rect;
                            }
                        }
                    }
                } else {
                    if min(y1, y2) < yp && yp < max(y1, y2) {
                        for w in min(xp, xq)..=max(xp, xq) {
                            if min(x1, x2) < w && w < max(x1, x2) {
                                continue 'next_rect;
                            }
                        }
                    }
                }
            }

            best = area;
        }
    }
    best
}

fn main() {
    let input = fs::read_to_string("dat/d9.txt").unwrap();
    let reds: Vec<(i64, i64)> = input.lines().map(|l| {let mut i = l.split(","); let x = i.next().unwrap().parse().unwrap(); (x, i.next().unwrap().parse().unwrap()) }).collect();

    println!("Part 1: {:?}", part1(&reds));
    println!("Part 2: {:?}", part2(&reds));
}
