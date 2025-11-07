use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p3.txt");

fn main() {
    println!("P1: {}", solve1::<4>(P1_INPUT));
    println!("P2: {}", solve2::<10>(P2_INPUT));
    println!("P3: {:?}", solve3::<5>(P3_INPUT, 256));
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Machine<const SIZE: usize> {
    counts: [usize; SIZE],
    wheels: [Vec<String>; SIZE],
    symbols: [u8; 256], //byte to index.
    symbol_count: usize,
}

impl<const SIZE: usize> Machine<SIZE> {
    fn from_str(input: &str) -> Self {
        let mut ls = input.lines();
        let counts: Vec<usize> = ls
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(counts.len(), SIZE);
        let _blank = ls.next().unwrap();
        let mut wheels = std::array::from_fn(|_| Vec::new());
        let mut symbols = [255u8; 256];
        let mut next_sym_ix = 0;
        for l in ls {
            let mut line = l.chars();
            let mut ix = 0;
            loop {
                let next: Option<String> = [line.next(), line.next(), line.next()]
                    .into_iter()
                    .collect();
                if let Some(x) = next {
                    for b in x.bytes().step_by(2).take(2) {
                        if symbols[b as usize] == 255 {
                            symbols[b as usize] = next_sym_ix.try_into().unwrap();
                            next_sym_ix += 1;
                        }
                    }
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
        Self {
            counts: counts.try_into().unwrap(),
            wheels,
            symbols,
            symbol_count: next_sym_ix,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MachineState<'m, const SIZE: usize> {
    machine: &'m Machine<SIZE>,
    pos: [u8; SIZE],
}
impl<const SIZE: usize> Hash for MachineState<'_, SIZE> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl<'m, const SIZE: usize> MachineState<'m, SIZE> {
    fn new(machine: &'m Machine<SIZE>) -> Self {
        assert!(machine.wheels.iter().all(|w| w.len() < u8::MAX as usize));
        Self {
            machine,
            pos: [0; SIZE],
        }
    }
    fn score(&self) -> usize {
        let mut map: Vec<usize> = std::iter::repeat_n(0, self.machine.symbol_count)
            .collect();
        // let mut map: BTreeMap<char, usize> = BTreeMap::new();
        for c in self
            .machine
            .wheels
            .iter()
            .zip(self.pos.iter())
            .flat_map(|(w, &p)| w[p as usize].bytes().step_by(2).take(2))
        {
            let ix = self.machine.symbols[c as usize];
            assert_ne!(ix, 255);
            map[ix as usize] += 1;
        }
        map.iter().map(|&x| x.saturating_sub(2)).sum()
    }
    fn pull_left(&self) -> Self {
        let mut new = self.clone();
        for i in 0..new.pos.len() {
            new.pos[i] = ((new.pos[i] as usize + 1) % new.machine.wheels[i].len())
                .try_into()
                .unwrap();
        }
        new
    }
    fn push_left(&self) -> Self {
        let mut new = self.clone();
        for i in 0..new.pos.len() {
            new.pos[i] = ((new.pos[i] as usize + new.machine.wheels[i].len() - 1)
                % new.machine.wheels[i].len()) as u8;
        }
        new
    }
    fn pull_right(&mut self, count: usize) {
        for i in 0..self.pos.len() {
            self.pos[i] = ((self.pos[i] as usize + count * self.machine.counts[i])
                % self.machine.wheels[i].len()) as u8;
        }
    }
    fn read(&self) -> String {
        self.pos
            .iter()
            .enumerate()
            .map(|(ix, &x)| &self.machine.wheels[ix][x as usize])
            .join(" ")
    }
}
fn solve1<const SIZE: usize>(input: &str) -> String {
    let m = Machine::<SIZE>::from_str(input);
    let mut ms = MachineState::new(&m);
    ms.pull_right(100);
    ms.read()
}

fn solve2<const SIZE: usize>(input: &str) -> usize {
    let m = Machine::<SIZE>::from_str(input);
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

fn solve3<const SIZE: usize>(input: &str, iterations: usize) -> (usize, usize) {
    //there is 94,500,000 positions for my input (35*30*40*45*50).
    let m = Machine::<SIZE>::from_str(input);
    let ms = MachineState::new(&m);
    let mut state: HashMap<MachineState<SIZE>, (usize, usize)> =
        [(ms, (0, 0))].into_iter().collect();
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
                new_bests
                    .entry(action)
                    .and_modify(|(min, max)| {
                        *min = new_min.min(*min);
                        *max = new_max.max(*max);
                    })
                    .or_insert((new_min, new_max));
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
        assert_eq!(solve1::<3>(EG1), ">.- -.- ^,-");
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve2::<3>(EG1), 280014668134)
    }
    const EG3: &str = "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>";
    #[test]
    fn p3_example() {
        assert_eq!(solve3::<3>(EG3, 1), (4, 1));
        assert_eq!(solve3::<3>(EG3, 2), (6, 1));
        assert_eq!(solve3::<3>(EG3, 3), (9, 2));
        assert_eq!(solve3::<3>(EG3, 10), (26, 5));
        assert_eq!(solve3::<3>(EG3, 100), (246, 50));
        assert_eq!(solve3::<3>(EG3, 256), (627, 128));
        assert_eq!(solve3::<3>(EG3, 1000), (2446, 500));
        assert_eq!(solve3::<3>(EG3, 2024), (4948, 1012));
    }
}
