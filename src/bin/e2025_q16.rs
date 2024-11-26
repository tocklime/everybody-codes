#![allow(dead_code)]
use std::{collections::BTreeMap, iter::repeat};

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p2.txt");
// const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q16_p3.txt");

fn main() {
    println!("P1: {}", solve1(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    // println!("P3: {}", solve::<3>(P3_INPUT));
}
struct Machine {
    counts: Vec<usize>,
    wheels: Vec<Vec<String>>
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
struct MachineState<'m> {
    machine: &'m Machine,
    pos: Vec<usize>,
}
impl<'m> MachineState<'m> {
    fn new(machine: &'m Machine) -> Self {
        Self {
            machine,
            pos: repeat(0).take(machine.counts.len()).collect()
        }
    }
    fn score(&self) -> usize {
        let mut map : BTreeMap<char, usize> = BTreeMap::new();
        for c in 
        self.machine.wheels
            .iter()
            .zip(self.pos.iter())
            .flat_map(|(w, &p)| [w[p].chars().nth(0).unwrap(), w[p].chars().nth(2).unwrap()]) {
                *map.entry(c).or_default() += 1;

            }
        map.values().map(|&x| x.saturating_sub(2)).sum()
    }
    fn pull_left(&mut self) {
        for i in 0..self.pos.len() {
            self.pos[i] = (self.pos[i] + 1) % self.machine.wheels[i].len();
        }
    }
    fn push_left(&mut self) {
        for i in 0..self.pos.len() {
            self.pos[i] = (self.pos[i] + (self.machine.wheels.len() - 1)) % self.machine.wheels[i].len();
        }
    }
    fn pull_right(&mut self, count: usize) {
        for i in 0..self.pos.len() {
            self.pos[i] = (self.pos[i] + count*self.machine.counts[i]) % self.machine.wheels[i].len();
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
        if ms.pos.iter().all(|&x| x==0) {
            let cycle_total : usize = coins.iter().sum();
            let cycle_size = coins.len();
            let complete_cycles = final_iter / cycle_size;
            let extra = final_iter % cycle_size;
            return complete_cycles * cycle_total + coins[0..extra].iter().sum::<usize>()
        }
    }
    coins.iter().sum::<usize>()
}

fn solve3(_input: &str) -> (usize,usize) {
    //there is 94,500,000 positions for my input (35*30*40*45*50).
    todo!()
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
    #[test]
    fn p3_example() {
        // assert_eq!(solve3(EG1), (627,128))
    }
}
