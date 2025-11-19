use std::collections::HashMap;

use everybody_codes::{collections::ToLookup, nums::NumBitExt};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q09_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q09_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q09_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn compare<A: Iterator<Item=char>,B : Iterator<Item=char>>(child: A, parent: B) -> u128 {
    let mut same_ixs = 0;
    for ((ix, c), p) in child.enumerate().zip(parent) {
        same_ixs.set_bit(ix.try_into().unwrap(), c == p);

    }
    same_ixs 
}
fn solve<const PART: usize>(input: &str) -> usize {
    let ls = input.lines().collect_vec();
    match PART {
        1 => {
            //child is 3 (from manual inspection)
            assert_eq!(ls.len(), 3);
            let p1_match = ls[0][2..]
                .chars()
                .zip(ls[2][2..].chars())
                .filter(|(a, b)| a == b)
                .count();
            let p2_match = ls[1][2..]
                .chars()
                .zip(ls[2][2..].chars())
                .filter(|(a, b)| a == b)
                .count();
            p1_match * p2_match
        }
        2 => {
            // let mut matches = HashMap::new();
            let len = ls[0][2..].len();
            let max_match = if len == 128 { u128::MAX } else {(1<<len) - 1};
            let mut total = 0;
            for child_ix in 0..ls.len() {
                let mut this_child_matches = HashMap::<usize, u128>::new();
                
                for parent_ix in 0..ls.len() {
                    if child_ix == parent_ix {
                        continue;
                    }
                    let same = compare(ls[child_ix].split_once(':').unwrap().1.chars(), ls[parent_ix].split_once(':').unwrap().1.chars());
                    let other_p = this_child_matches.iter().find(|&(_,v)| v | same == max_match);
                    if let Some(other_p) = other_p {
                        //found parent.
                        total += (same.count_ones() * other_p.1.count_ones()) as usize;
                        break;
                    }
                    *this_child_matches.entry(parent_ix).or_default() = same;
                }
            }
            total
            // 0
        }
        3 => {
            let len = ls[0][2..].len();
            let max_match = if len == 128 { u128::MAX } else {(1<<len) - 1};
            let mut connections = Vec::new();
            for child_ix in 0..ls.len() {
                let mut this_child_matches = HashMap::new();
                
                for parent_ix in 0..ls.len() {
                    if child_ix == parent_ix {
                        continue;
                    }
                    let (cname, cdna) = ls[child_ix].split_once(':').unwrap();
                    let (pname, pdna) = ls[parent_ix].split_once(':').unwrap();
                    let same = compare(cdna.chars(), pdna.chars());
                    let other_p = this_child_matches.iter().find(|&(k,v)| v | same == max_match && k != &child_ix);
                    if let Some((&other_p_ix, _)) = other_p {
                        //found parent.
                        let cid: u32 = cname.parse().unwrap();
                        let pid: u32 = pname.parse().unwrap();

                        let op : &str = ls[other_p_ix];
                        let opid: u32 = op.split_once(':').unwrap().0.parse::<u32>().unwrap();
                        connections.push(vec![cid,pid,opid]);
                        // dbg!(cid, pid, opid);
                        break;
                    }
                    *this_child_matches.entry(parent_ix).or_default() = same;
                }
            }
            let x = pathfinding::prelude::separate_components(&connections);
            let lu = x.0.iter().map(|(&&elem, &set)| (set,elem)).collect_lookup();
            lu.values().map(|x| x.iter().sum::<u32>()).max().unwrap() as usize
        }
        _ => unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG";
    const EG2: &str = "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG";
    const EG3: &str = "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 414);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG2), 1245);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 36);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 6048);
        assert_eq!(solve::<2>(P2_INPUT), 323138);
        assert_eq!(solve::<3>(P3_INPUT), 45863);
    }
}

//TODO: Proper parsing, refactoring.