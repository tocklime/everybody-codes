use std::{cmp::Reverse, collections::BTreeMap};

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q01_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q01_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q01_p3.txt");

fn main() {
    println!("P1: {}", p1(P1_INPUT));
    println!("P2: {}", p2(P2_INPUT));
    println!("P3: {}", p3(P3_INPUT));
}
struct Scale {
    id: usize,
    cols: [u32; 3],
    shininess: u32,
}
impl Scale {
    fn main_col_ix(&self) -> Option<usize> {
        let max_val = self.cols.iter().max().unwrap();
        let max_count = self.cols.iter().filter(|x| x == &max_val).count();
        (max_count == 1).then_some(self.cols.iter().position_max().unwrap())
    }
    fn brightness(&self) -> u32 {
        self.cols.iter().sum()
    }
    fn is_shiny(&self) -> Option<bool> {
        match self.shininess {
            0..=30 => Some(false),
            33.. => Some(true),
            _ => None,
        }
    }
}
fn generate(input: &str) -> Vec<Scale> {
    input
        .lines()
        .map(|l| {
            let (id, rest) = l.split_once(':').unwrap();
            let id = str::parse(id).unwrap();
            let words = rest.split(' ');
            let vals = words
                .map(|w| {
                    w.chars()
                        .rev()
                        .enumerate()
                        .map(|(ix, c)| if c.is_uppercase() { 1 << ix } else { 0 })
                        .sum::<u32>()
                })
                .collect::<Vec<u32>>();
            let (cols, rest) = vals.split_at(3);
            Scale {
                id,
                cols: cols.try_into().unwrap(),
                shininess: rest.first().copied().unwrap_or_default(),
            }
        })
        .collect()
}
fn p1(input: &str) -> usize {
    generate(input)
        .into_iter()
        .map(|s| s.main_col_ix().and_then(|x| (x == 1).then_some(s.id)).unwrap_or_default())
        .sum()
}
fn p2(input: &str) -> usize {
    generate(input)
        .iter()
        .max_by_key(|x| (x.shininess, Reverse(x.brightness())))
        .unwrap().id
}
fn p3(input: &str) -> usize {
    let mut grouped: BTreeMap<(usize, bool), Vec<usize>> = BTreeMap::new();
    for s in generate(input) {
        if let (Some(ix), Some(shiny)) = (s.main_col_ix(), s.is_shiny()) {
            grouped.entry((ix, shiny)).or_default().push(s.id);
        }
    }
    grouped.into_values().max_by_key(Vec::len).unwrap().iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "2456:rrrrrr ggGgGG bbbbBB
7689:rrRrrr ggGggg bbbBBB
3145:rrRrRr gggGgg bbbbBB
6710:rrrRRr ggGGGg bbBBbB";
    #[test]
    fn p1_example() {
        assert_eq!(p1(EG1), 9166);
    }
    const EG2: &str = "2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss";
    #[test]
    fn p2_example() {
        assert_eq!(p2(EG2), 2456);
    }
    const EG3: &str = "15437:rRrrRR gGGGGG BBBBBB sSSSSS
94682:RrRrrR gGGggG bBBBBB ssSSSs
56513:RRRrrr ggGGgG bbbBbb ssSsSS
76346:rRRrrR GGgggg bbbBBB ssssSs
87569:rrRRrR gGGGGg BbbbbB SssSss
44191:rrrrrr gGgGGG bBBbbB sSssSS
49176:rRRrRr GggggG BbBbbb sSSssS
85071:RRrrrr GgGGgg BBbbbb SSsSss
44303:rRRrrR gGggGg bBbBBB SsSSSs
94978:rrRrRR ggGggG BBbBBb SSSSSS
26325:rrRRrr gGGGgg BBbBbb SssssS
43463:rrrrRR gGgGgg bBBbBB sSssSs
15059:RRrrrR GGgggG bbBBbb sSSsSS
85004:RRRrrR GgGgGG bbbBBB sSssss
56121:RRrRrr gGgGgg BbbbBB sSsSSs
80219:rRRrRR GGGggg BBbbbb SssSSs";
    #[test]
    fn p3_example() {
        assert_eq!(p3(EG3), 292320);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(p1(P1_INPUT), 61110);
        assert_eq!(p2(P2_INPUT), 17039);
        assert_eq!(p3(P3_INPUT), 10578316);
    }
}
