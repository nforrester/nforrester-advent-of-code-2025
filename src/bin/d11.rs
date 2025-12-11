use std::fs;
use std::collections::HashMap;

fn parse_name(name: &str) -> u32 {
    let mut nb = name.bytes();
    let b1 = nb.next().unwrap() as u32;
    let b2 = nb.next().unwrap() as u32;
    let b3 = nb.next().unwrap() as u32;
    (b1 << 16) + (b2 << 8) + b3
}

fn parse_in(input: &str) -> HashMap<u32, Vec<u32>> {
    input.lines().map(|l| {
        let mut sc = l.split(": ");
        let fm = parse_name(sc.next().unwrap());
        let to = sc.next().unwrap().split(" ").map(|w| parse_name(w)).collect();
        (fm, to)
    }).collect()
}

fn count_paths(graph: &HashMap<u32, Vec<u32>>, start: u32, end: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if start == end {
        return 1;
    }
    if memo.contains_key(&start) {
        return *memo.get(&start).unwrap();
    }
    if let Some(v) = graph.get(&start) {
        let x = v.iter().map(|s| count_paths(graph, *s, end, memo)).sum();
        memo.insert(start, x);
        return x;
    }
    memo.insert(start, 0);
    return 0;
}

fn main() {
    {
        let input = fs::read_to_string("dat/d11.txt").unwrap();
        let graph = parse_in(&input);
        let you = parse_name("you");
        let out = parse_name("out");

        let mut memo = HashMap::new();

        println!("Part 1: {:?}", count_paths(&graph, you, out, &mut memo));
    }

    {
        let input = fs::read_to_string("dat/d11.txt").unwrap();
        let graph = parse_in(&input);
        let out = parse_name("out");
        let svr = parse_name("svr");
        let dac = parse_name("dac");
        let fft = parse_name("fft");

        let mut memo = HashMap::new();
        let sd = count_paths(&graph, svr, dac, &mut memo);
        memo = HashMap::new();
        let sf = count_paths(&graph, svr, fft, &mut memo);

        memo = HashMap::new();
        let fd = count_paths(&graph, fft, dac, &mut memo);
        memo = HashMap::new();
        let df = count_paths(&graph, dac, fft, &mut memo);

        memo = HashMap::new();
        let f0 = count_paths(&graph, fft, out, &mut memo);
        memo = HashMap::new();
        let d0 = count_paths(&graph, dac, out, &mut memo);

        println!("Part 2: {:?}", sd * df * f0 + sf * fd * d0);
    }
}
