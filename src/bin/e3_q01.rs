use std::collections::BTreeMap;

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q01_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q01_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q01_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let mut p2_vals = Vec::new();
    let mut p3_vals : BTreeMap<(usize, bool), Vec<usize>> = BTreeMap::new();
    let mut ans = 0;
    for l in input.lines() {
        let (id, rest) = l.split_once(':').unwrap();
        let id = usize::from_str_radix(id, 10).unwrap();
        let words = rest.split(' ').collect::<Vec<&str>>();
        let vals = words.into_iter().map(|w| w.chars().rev().enumerate().map(|(ix, c)| if c.is_uppercase() {
            1<<ix
        } else { 0}).sum::<u32>()).collect::<Vec<u32>>();
        let max = vals[0..3].iter().max().unwrap();
        let max_count = vals[0..3].iter().filter(|x| x == &max).count();
        let max_pos = vals[0..3].iter().position_max().unwrap();
        match PART {
            1 => {
                if max_count == 1 && max_pos == 1 {
                    ans += id;
                }
            }
            2 => {
                p2_vals.push((id, vals));
            }
            3 => {
                if !(31..33).contains(&vals[3]) && max_count == 1{
                    let is_shiny = vals[3] >= 33;
                    p3_vals.entry((max_pos, is_shiny)).or_default().push(id);
                }
            }
            _ => unimplemented!()
        }
    }
    if PART == 2 {
        let best_shine = p2_vals.iter().max_by_key(|x| (x.1[3], u32::MAX-(x.1[0..2].iter().sum::<u32>()))).unwrap();
        return best_shine.0;
    }
    if PART == 3 {
        let biggest = p3_vals.iter().max_by_key(|x| x.1.len()).unwrap();
        return biggest.1.iter().sum::<usize>();
    }
    ans
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "2456:rrrrrr ggGgGG bbbbBB
7689:rrRrrr ggGggg bbbBBB
3145:rrRrRr gggGgg bbbbBB
6710:rrrRRr ggGGGg bbBBbB";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 9166);
    }
    const EG2 : &str = "2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss";
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG2), 2456);
    }
    const EG3 : &str = "15437:rRrrRR gGGGGG BBBBBB sSSSSS
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
        assert_eq!(solve::<3>(EG3), 292320);
    }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}
