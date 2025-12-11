use std::fs;
use std::iter::zip;
use std::cmp::min;

#[derive(Debug)]
struct Machine {
    target_lights: u16,
    buttons: Vec<u16>,
    target_joltages: Vec<u16>,
}

fn parse_machine(line: &str) -> Machine {
    let mut words: Vec<&str> = line.split_whitespace().collect();
    let wl = words.len();
    let lights_word = words[0];
    let joltage_word = words[wl-1];
    let button_words = &mut words[1..wl-1];

    let lights_chars: Vec<char> = lights_word.chars().collect();
    let lights_bools = lights_chars[1..lights_chars.len()-1].iter().map(|c| *c == '#');
    let mut target_lights = 0;
    for lb in lights_bools.rev() {
        target_lights = target_lights << 1;
        if lb {
            target_lights = target_lights | 1;
        }
    }

    button_words.sort_by_key(|bw| -1 * bw.len() as i64);
    let buttons = button_words.iter().map(|w| {
        let wires: Vec<u16> = w.split("(").collect::<String>().split(")").collect::<String>().split(",").map(|n| n.parse().unwrap()).collect();
        let mut button = 0;
        for i in wires {
            button = button | (1 << i);
        }
        button
    }).collect();

    let target_joltages = joltage_word.split("{").collect::<String>().split("}").collect::<String>().split(",").map(|w| w.parse().unwrap()).collect();

    Machine { target_lights, buttons, target_joltages }
}

fn init_machine_in_steps(steps: u16, start: u16, goal: u16, toggles: &Vec<u16>) -> bool {
    if steps == 0 {
        return start == goal;
    }
    toggles.iter().any(|t| init_machine_in_steps(steps-1, start ^ t, goal, toggles))
}

fn init_machine(m: &Machine) -> u16 {
    for steps in 0.. {
        if init_machine_in_steps(steps, 0, m.target_lights, &m.buttons) {
            return steps;
        }
    }
    panic!();
}

fn jolt_machine_in_steps(steps: u16, start: Vec<u16>, goal: &Vec<u16>, toggles: &[u16]) -> Option<u16> {
    for i in 0..goal.len() {
        if start[i] > goal[i] {
            return None;
        }
    }
    if start == *goal {
        return Some(steps);
    }

    let error: Vec<u16> = zip(goal, &start).map(|(g, s)| g - *s).collect();
    let mut max_progress_and_toggle_index: Vec<(_, usize)> = (0..toggles.len()).map(|ti| {
        let mut max_clicks = 0xffffu16;
        let mut wires = 0;
        let mut i = 0;
        let mut t = toggles[ti];
        while t > 0u16 {
            if t & 1 == 1 {
                max_clicks = min(max_clicks, error[i]);
                wires += 1;
            }
            t = t >> 1;
            i += 1;
        }
        ((max_clicks, 0xffffu16 - wires), ti)
    }).collect();
    max_progress_and_toggle_index.sort();

    max_progress_and_toggle_index.iter().flat_map(|(_, ti)| {
        let mut new = start.clone();
        let mut i = 0;
        let mut t = toggles[*ti];
        while t > 0u16 {
            new[i] += t & 1;
            t = t >> 1;
            i += 1;
        }
        jolt_machine_in_steps(steps+1, new, goal, &toggles[*ti..])
    }).next()
}

fn jolt_machine(m: &Machine) -> u16 {
    println!("Solving {:?}", m);
    jolt_machine_in_steps(0, vec![0u16; m.target_joltages.len()], &m.target_joltages, &m.buttons).unwrap()
}

fn main() {
    {
        let input = fs::read_to_string("dat/d10.test.txt").unwrap();
        let machines: Vec<Machine> = input.lines().map(|l| parse_machine(l)).collect();

        println!("Part 1: {:?}", machines.iter().map(|m| init_machine(m)).sum::<u16>());
        println!("Part 2: {:?}", machines.iter().map(|m| jolt_machine(m)).sum::<u16>());
    }
    {
        let input = fs::read_to_string("dat/d10.txt").unwrap();
        let machines: Vec<Machine> = input.lines().map(|l| parse_machine(l)).collect();

        println!("Part 1: {:?}", machines.iter().map(|m| init_machine(m)).sum::<u16>());
        println!("Part 2: {:?}", machines.iter().map(|m| jolt_machine(m)).sum::<u16>());
    }
}
