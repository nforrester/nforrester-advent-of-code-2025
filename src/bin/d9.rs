use std::fs;
use std::cmp::{max, min};
use itertools::{sorted, Itertools};

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

struct Uniq<I: Sized + Iterator> {
    it: I,
    prev: Option<I::Item>,
}

trait Uniqable {
    fn uniq(self) -> Uniq<Self> where Self: Sized + Iterator;
}

impl<I: Iterator> Uniqable for I {
    fn uniq(self) -> Uniq<Self> {
        Uniq::<Self> {
            it: self,
            prev: None,
        }
    }
}

impl<T: PartialEq + Copy, I: Iterator<Item=T>> Iterator for Uniq<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.it.next() {
            if let Some(p) = &self.prev {
                if *p == n {
                    return None;
                }
                self.prev = Some(n);
                return Some(n);
            }
            return Some(n);
        }
        return None;
    }
}

fn part2(reds: &Vec<(i64, i64)>) -> i64 {
    let edges: Vec<_> = (0..reds.len()).map(|i| (reds[i], reds[(i+1) % reds.len()])).collect();

    let edges_h: Vec<(i64, (i64, i64))> = sorted(edges.iter().flat_map(|((x1, y), (x2, _))| if x1 == x2 { Some((*y, (*min(x1, x2), *max(x1, x2)))) } else { None })).collect();
    let edges_v: Vec<(i64, (i64, i64))> = sorted(edges.iter().flat_map(|((x, y1), (_, y2))| if y1 == y2 { Some((*x, (*min(y1, y2), *max(y1, y2)))) } else { None })).collect();

    let uniq_xs: Vec<_> = sorted(reds.iter().flat_map(|(x, _)| vec![x-1, *x, x+1])).uniq().collect();
    let uniq_ys: Vec<_> = sorted(reds.iter().flat_map(|(_, y)| vec![y-1, *y, y+1])).uniq().collect();

    let good_ranges_uniq_xs: Vec<Vec<(i64, i64)>> = uniq_xs.iter().map(|x| edges_h.iter().flat_map(|(y, (x1, x2))| if x1 <= x && x <= x2 { Some(y) } else { None }).chunks(2).into_iter().map(|mut i| (*i.next().unwrap(), *i.next().unwrap())).collect()).collect();

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

fn main() {
    let input = fs::read_to_string("dat/d9.test.txt").unwrap();
    let reds: Vec<(i64, i64)> = input.lines().map(|l| {let mut i = l.split(","); let x = i.next().unwrap().parse().unwrap(); (x, i.next().unwrap().parse().unwrap()) }).collect();

    println!("Part 1: {:?}", part1(&reds));
    println!("Part 2: {:?}", part2(&reds));
}
