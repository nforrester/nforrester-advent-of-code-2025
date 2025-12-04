use std::fs;

fn parse_grid(s: &str) -> Vec<Vec<bool>> {
    s.lines().map(|l| l.chars().map(|c| c == '@').collect()).collect()
}

fn check(grid: &Vec<Vec<bool>>, i: i64, j: i64) -> bool {
    if i < 0 {
        return false;
    }
    if i >= grid.len() as i64 {
        return false;
    }
    if j < 0 {
        return false;
    }
    if j >= grid[0].len() as i64 {
        return false;
    }
    grid[i as usize][j as usize]
}

fn count_accessible(grid: &Vec<Vec<bool>>) -> i64 {
    let mut result = 0;
    for i in 0..grid.len() as i64 {
        for j in 0..grid[0].len() as i64 {
            if !check(grid, i, j) {
                continue;
            }
            let mut adjacent = 0;
            for p in -1..=1 {
                for q in -1..=1 {
                    if p == 0 && q == 0 {
                        continue;
                    }
                    if check(grid, i + p, j + q) {
                        adjacent += 1;
                    }
                }
            }
            if adjacent < 4 {
                result += 1;
            }
        }
    }
    result
}

fn count_removable(mut grid: Vec<Vec<bool>>) -> i64 {
    let mut result = 0;
    let mut removed_any = true;
    while removed_any {
        removed_any = false;
        for i in 0..grid.len() as i64 {
            for j in 0..grid[0].len() as i64 {
                if !check(&grid, i, j) {
                    continue;
                }
                let mut adjacent = 0;
                for p in -1..=1 {
                    for q in -1..=1 {
                        if p == 0 && q == 0 {
                            continue;
                        }
                        if check(&grid, i + p, j + q) {
                            adjacent += 1;
                        }
                    }
                }
                if adjacent < 4 {
                    result += 1;
                    removed_any = true;
                    grid[i as usize][j as usize] = false;
                }
            }
        }
    }
    result
}

fn main() {
    let input: String = fs::read_to_string("dat/d4.txt").unwrap();
    let grid = parse_grid(&input);
    let accessible = count_accessible(&grid);
    println!("Part 1: {}", accessible);
    let removable = count_removable(grid);
    println!("Part 2: {}", removable);
}
