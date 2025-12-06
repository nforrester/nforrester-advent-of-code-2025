use std::fs;
use std::iter::zip;
use std::mem::swap;
use std::cell::RefCell;

///////////////////////////////////////////////////////////////////

fn transpose<T>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if rows.len() == 0 {
        return rows;
    }

    let mut rows_it = rows.into_iter();
    let seed: Vec<Vec<T>> = rows_it.next().unwrap().into_iter().map(|x: T| -> Vec<T> { vec![x] }).collect();
    rows_it.fold(seed, |transposed: Vec<Vec<T>>, row: Vec<T>| -> Vec<Vec<T>> { zip(transposed.into_iter(), row.into_iter()).map(|(mut xs, x) : (Vec<T>, T)| -> Vec<T> { xs.push(x); xs }).collect() })
}

///////////////////////////////////////////////////////////////////

struct SplitLast<'last, T, I: Iterator<Item=T>> {
    it: I,
    next: Option<T>,
    last: &'last RefCell<Option<T>>,
}

fn split_last<'last, T, I: Iterator<Item=T>>(mut it: I, last: &'last RefCell<Option<T>>) -> SplitLast<'last,T, I> {
    let next = it.next();
    SplitLast {
        it,
        next,
        last,
    }
}

impl<'last, T, I: Iterator<Item=T>> Iterator for SplitLast<'last, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.it.next() {
            let mut this = Some(n);
            swap(&mut this, &mut self.next);
            return this;
        }
        let mut last = None;
        swap(&mut last, &mut self.next);
        self.last.replace(last);
        return None;
    }
}

///////////////////////////////////////////////////////////////////

type Op = fn(&mut dyn Iterator<Item = i64>) -> i64;

fn parse_op(c: char) -> Op {
    match c {
        '+' => |it| it.sum(),
        '*' => |it| it.product(),
        _ => panic!(),
    }
}

fn part1(file: &str) -> i64 {
    let input: String = fs::read_to_string(file).unwrap();
    let last_words = RefCell::new(None);
    let lines_words = split_last(input.lines().map(|l| l.split_whitespace()), &last_words);
    let args_rows: Vec<Vec<i64>> = lines_words.map(|lw| lw.map(|w| w.parse().unwrap()).collect()).collect();
    let ops = last_words.take().unwrap().map(|w| parse_op(w.chars().next().unwrap()));

    let args_cols = transpose(args_rows);
    zip(args_cols, ops).map(|(args, op)| op(&mut args.into_iter())).sum()
}

struct Problem {
    args: Vec<i64>,
    op: Op,
}

fn parse_problems(mut problems: Vec<Option<Problem>>, mut col: Vec<char>) -> Vec<Option<Problem>> {
    if let Some(Some(last_problem)) = problems.last_mut() {
        let num_string: String = col.iter().filter(|c| **c != ' ').collect();
        if num_string == "" {
            problems.push(None);
        } else {
            last_problem.args.push(num_string.parse().unwrap());
        }
    } else {
        let op = parse_op(col.pop().unwrap());
        let num_string: String = col.iter().filter(|c| **c != ' ').collect();
        let n = num_string.parse().unwrap();

        problems.pop();
        problems.push(Some(Problem {
            args: vec![n],
            op
        }));
    }
    problems
}

fn part2(file: &str) -> i64 {
    let input: String = fs::read_to_string(file).unwrap();
    let in_lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_line_len = in_lines.iter().map(|l| l.len()).max().unwrap();
    let columns = transpose(in_lines.into_iter().map(|mut l| { l.resize(max_line_len, ' ') ; l }).collect());
    let problems = columns.into_iter().fold(vec![None], |ps, c| parse_problems(ps, c)).into_iter().map(|x| x.unwrap());
    problems.map(|p| (p.op)(&mut p.args.into_iter())).sum()
}

fn main() {
    let file = "dat/d6.txt";
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}
