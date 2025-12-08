use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct V3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromIterator<f64> for V3 {
    fn from_iter<I: IntoIterator<Item=f64>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        let x = i.next().unwrap();
        let y = i.next().unwrap();
        let z = i.next().unwrap();
        assert!(i.next() == None);
        V3 { x, y, z }
    }
}

impl V3 {
    fn dist(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

struct Pairs<'a, T> {
    v: &'a Vec<T>,
    i: usize,
    j: usize,
}

fn pairs<'a, T>(v: &'a Vec<T>) -> Pairs<'a, T> {
    Pairs { v, i: 0, j: 1 }
}

impl<'a, T: Copy> Iterator for Pairs<'a, T> {
    type Item = ((T, usize), (T, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.j < self.v.len() {
            let j = self.j;
            self.j += 1;
            return Some(((self.v[self.i], self.i), (self.v[j], j)));
        }
        self.i += 1;
        if self.i < self.v.len() {
            self.j = self.i + 1;
            return self.next();
        }
        return None;
    }
}

fn part1(n: usize, boxes: &Vec<V3>) -> usize {
    let mut boxes_in_circuits: Vec<HashSet<usize>> = (0..boxes.len()).map(|i| { let mut s = HashSet::new(); s.insert(i); s }).collect();
    let mut circuit_containing_box: Vec<usize> = (0..boxes.len()).collect();

    let mut db: Vec<_> = pairs(boxes).map(|((a, i), (b, j))| (a.dist(b), (a, i), (b, j))).collect();
    db.sort_by(|(a, _, _), (b, _, _)| f64::total_cmp(a, b));
    let joined_pairs = db.iter().take(n).map(|(_, (_, i), (_, j))| (i, j));

    for (i, j) in joined_pairs {
        if circuit_containing_box[*i] == circuit_containing_box[*j] {
            continue;
        }

        let to;
        let fm;
        if circuit_containing_box[*i] < circuit_containing_box[*j] {
            to = circuit_containing_box[*i];
            fm = circuit_containing_box[*j];
        } else {
            to = circuit_containing_box[*j];
            fm = circuit_containing_box[*i];
        }

        let moved_boxes: Vec<usize> = boxes_in_circuits[fm].iter().map(|x| *x).collect();
        for b in moved_boxes {
            boxes_in_circuits[to].insert(b);
            circuit_containing_box[b] = to;
        }
        boxes_in_circuits[fm].clear();
    }

    boxes_in_circuits.sort_by(|a, b| usize::cmp(&b.len(), &a.len()));
    boxes_in_circuits.iter().take(3).map(|s| s.len()).product()
}

fn part2(boxes: &Vec<V3>) -> usize {
    let mut boxes_in_circuits: Vec<HashSet<usize>> = (0..boxes.len()).map(|i| { let mut s = HashSet::new(); s.insert(i); s }).collect();
    let mut circuit_containing_box: Vec<usize> = (0..boxes.len()).collect();

    let mut db: Vec<_> = pairs(boxes).map(|((a, i), (b, j))| (a.dist(b), (a, i), (b, j))).collect();
    db.sort_by(|(a, _, _), (b, _, _)| f64::total_cmp(a, b));
    let joined_pairs = db.iter().map(|(_, (_, i), (_, j))| (i, j));

    for (i, j) in joined_pairs {
        if circuit_containing_box[*i] == circuit_containing_box[*j] {
            continue;
        }

        let to;
        let fm;
        if circuit_containing_box[*i] < circuit_containing_box[*j] {
            to = circuit_containing_box[*i];
            fm = circuit_containing_box[*j];
        } else {
            to = circuit_containing_box[*j];
            fm = circuit_containing_box[*i];
        }

        let moved_boxes: Vec<usize> = boxes_in_circuits[fm].iter().map(|x| *x).collect();
        for b in moved_boxes {
            boxes_in_circuits[to].insert(b);
            circuit_containing_box[b] = to;
        }
        boxes_in_circuits[fm].clear();

        if boxes_in_circuits[0].len() == boxes.len() {
            return (boxes[*i].x as usize) * (boxes[*j].x as usize);
        }
    }
    panic!();
}

fn main() {
    let input = fs::read_to_string("dat/d8.txt").unwrap();
    let boxes: Vec<V3> = input.lines().map(|l| l.split(",").map(|w| w.parse().unwrap()).collect()).collect();

    println!("Part 1: {:?}", part1(1000, &boxes));
    println!("Part 2: {:?}", part2(&boxes));
}
