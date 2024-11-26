use std::{
    collections::{BTreeMap, HashMap},
    iter::repeat,
};

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p3.txt");

fn main() {
    println!("P1: {}", solve1(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    println!("P3: {:?}", solve3(P3_INPUT, 256));
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Machine {
    counts: Vec<usize>,
    wheels: Vec<Vec<String>>,
}
impl Machine {
    fn from_str(input: &str) -> Self {
        let mut ls = input.lines();
        let counts: Vec<usize> = ls
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let _blank = ls.next().unwrap();
        let mut wheels = vec![Vec::new(); counts.len()];
        for l in ls {
            let mut line = l.chars();
            let mut ix = 0;
            loop {
                let next: Option<String> = [line.next(), line.next(), line.next()]
                    .into_iter()
                    .collect();
                if let Some(x) = next {
                    if x != "   " {
                        wheels[ix].push(x);
                    }
                } else {
                    break;
                }
                line.next(); //space.
                ix += 1;
            }
        }
        Self { counts, wheels }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MachineState<'m> {
    machine: &'m Machine,
    pos: Vec<usize>,
}
impl<'m> MachineState<'m> {
    fn new(machine: &'m Machine) -> Self {
        Self {
            machine,
            pos: repeat(0).take(machine.counts.len()).collect(),
        }
    }
    fn score(&self) -> usize {
        let mut map: BTreeMap<char, usize> = BTreeMap::new();
        for c in self
            .machine
            .wheels
            .iter()
            .zip(self.pos.iter())
            .flat_map(|(w, &p)| [w[p].chars().nth(0).unwrap(), w[p].chars().nth(2).unwrap()])
        {
            *map.entry(c).or_default() += 1;
        }
        map.values().map(|&x| x.saturating_sub(2)).sum()
    }
    fn pull_left(&self) -> Self {
        let mut new = self.clone();
        for i in 0..new.pos.len() {
            new.pos[i] = (new.pos[i] + 1) % new.machine.wheels[i].len();
        }
        new
    }
    fn push_left(&self) -> Self {
        let mut new = self.clone();
        for i in 0..new.pos.len() {
            new.pos[i] =
                (new.pos[i] + (new.machine.wheels[i].len() - 1)) % new.machine.wheels[i].len();
        }
        new
    }
    fn pull_right(&mut self, count: usize) {
        for i in 0..self.pos.len() {
            self.pos[i] =
                (self.pos[i] + count * self.machine.counts[i]) % self.machine.wheels[i].len();
        }
    }
    fn read(&self) -> String {
        self.pos
            .iter()
            .enumerate()
            .map(|(ix, &x)| &self.machine.wheels[ix][x])
            .join(" ")
    }
}
fn solve1(input: &str) -> String {
    let m = Machine::from_str(input);
    let mut ms = MachineState::new(&m);
    ms.pull_right(100);
    ms.read()
}

fn solve2(input: &str) -> usize {
    let m = Machine::from_str(input);
    let mut ms = MachineState::new(&m);

    let final_iter = 202420242024_usize;
    let mut coins = Vec::new();
    for _i in 0..final_iter {
        ms.pull_right(1);
        coins.push(ms.score());
        if ms.pos.iter().all(|&x| x == 0) {
            let cycle_total: usize = coins.iter().sum();
            let cycle_size = coins.len();
            let complete_cycles = final_iter / cycle_size;
            let extra = final_iter % cycle_size;
            return complete_cycles * cycle_total + coins[0..extra].iter().sum::<usize>();
        }
    }
    coins.iter().sum::<usize>()
}

fn solve3(input: &str, iterations: usize) -> (usize, usize) {
    //there is 94,500,000 positions for my input (35*30*40*45*50).
    let m = Machine::from_str(input);
    let ms = MachineState::new(&m);
    let mut state: HashMap<MachineState, (usize, usize)> = [(ms, (0, 0))].into_iter().collect();
    for _n in 0..iterations {
        let mut new_bests = HashMap::new();
        for (k, v) in &state {
            let push = k.push_left();
            let pull = k.pull_left();
            let neither = k.clone();
            for mut action in [push, pull, neither].into_iter() {
                action.pull_right(1);
                let coins = action.score();
                let new_min = v.0 + coins;
                let new_max = v.1 + coins;
                let existing = new_bests.entry(action).or_insert((usize::MAX, 0usize));
                existing.0 = existing.0.min(new_min);
                existing.1 = existing.1.max(new_max);
            }
        }
        state = new_bests;
    }
    let min = state.values().map(|x| x.0).min().unwrap();
    let max = state.values().map(|x| x.1).max().unwrap();
    (max, min)
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>";
    #[test]
    fn p1_example() {
        assert_eq!(solve1(EG1), ">.- -.- ^,-");
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve2(EG1), 280014668134)
    }
    const EG3: &str = "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>";
    #[test]
    fn p3_example() {
        assert_eq!(solve3(EG3, 1), (4, 1));
        assert_eq!(solve3(EG3, 2), (6, 1));
        assert_eq!(solve3(EG3, 3), (9, 2));
        assert_eq!(solve3(EG3, 10), (26, 5));
        assert_eq!(solve3(EG3, 100), (246, 50));
        assert_eq!(solve3(EG3, 256), (627, 128));
        assert_eq!(solve3(EG3, 1000), (2446, 500));
        assert_eq!(solve3(EG3, 2024), (4948, 1012));
    }
}
