use std::cmp::Ordering;

use nom::{branch::alt, bytes::complete::tag, Parser};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q02_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q02_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q02_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
#[derive(Debug)]
struct Tree {
    id: u32,
    rank: u32,
    label: char,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}
impl Tree {
    fn new(id: u32, rank: u32, label: char) -> Self {
        Self {
            id,
            rank,
            label,
            left: None,
            right: None,
        }
    }
    fn find_mut(&mut self, id: u32) -> Vec<&mut Self> {
        if self.id == id {
            vec![self]
        } else {
            let mut left = self.left.as_mut().map(|t| t.find_mut(id)).unwrap_or_default();
            let right =  self.right.as_mut().map(|t| t.find_mut(id)).unwrap_or_default();
            left.extend(right);
            left
        }
    }
    fn depth(&self) -> u32 {
        let l_depth = self.left.as_ref().map(|t| t.depth()).unwrap_or_default();
        let r_depth = self.right.as_ref().map(|t| t.depth()).unwrap_or_default();
        1 + l_depth.max(r_depth)
    }
    fn read_rank(&self, n: u32) -> Vec<char> {
        if n == 0 {
            vec![self.label]
        } else {
            let mut left = self
                .left
                .as_ref()
                .map(|t| t.read_rank(n - 1))
                .unwrap_or_default();
            let right = self
                .right
                .as_ref()
                .map(|t| t.read_rank(n - 1))
                .unwrap_or_default();
            left.extend(right);
            left
        }
    }

    fn read_widest_rank(&self) -> String {
        let max = self.depth();
        let x = (0..=max)
            .rev()
            .map(|n| self.read_rank(n))
            .max_by_key(|x| x.len())
            .unwrap();
        x.into_iter().collect()
    }
    fn insert(&mut self, other: Self) {
        match self.rank.cmp(&other.rank) {
            Ordering::Greater => {
                if let Some(t) = &mut self.left {
                    t.insert(other);
                } else {
                    self.left = Some(Box::new(other));
                }
            }
            Ordering::Equal => unreachable!(),
            Ordering::Less => {
                if let Some(t) = &mut self.right {
                    t.insert(other);
                } else {
                    self.right = Some(Box::new(other));
                }
            }
        }
    }
    fn parse<'a>(id: u32) -> impl Parser<&'a str, Output = Self, Error = Err<'a>> {
        (
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::anychar,
        )
            .map(move |(a, _, b)| Self::new(id, a, b))
    }
}

type Err<'a> = nom::error::Error<&'a str>;

#[derive(Debug)]
enum Instruction {
    Add { left: Tree, right: Tree },
    Swap(u32),
}

impl Instruction {
    fn lr_value<'a>(
        id: u32, 
        tag_name: &'static str,
    ) -> impl Parser<&'a str, Output = Tree, Error = Err<'a>> {
        (tag(tag_name), tag("=["), Tree::parse(id), tag("]")).map(|(_, _, t, _)| t)
    }
    fn parse_add<'a>() -> impl Parser<&'a str, Output = Self, Error = Err<'a>> {
        |input: &'a str| {
            let (input, _) = tag("ADD id=")(input)?;
            let (input, id) = nom::character::complete::u32(input)?;
            let (input, (left,right)) = (
                Self::lr_value(id, " left"),
                Self::lr_value(id, " right"),
            ).parse(input)?;
            Ok((input, Self::Add { left, right }))
        }
    }
    fn parse_swap<'a>() -> impl Parser<&'a str, Output = Self, Error = Err<'a>> {
        (tag("SWAP "), nom::character::complete::u32).map(|(_, i)| Self::Swap(i))
    }
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = Err<'a>> {
        alt((Self::parse_add(), Self::parse_swap()))
    }
}

fn solve<const PART: usize>(input: &str) -> String {
    let mut top_tree = Tree::new(0, 0, '_');
    // let mut top_l: Option<Tree> = None;
    // let mut top_r: Option<Tree> = None;
    for l in input.trim().lines() {
        let (_, i) = Instruction::parse()
            .parse(l)
            .ok()
            .unwrap_or_else(|| panic!("Can't parse {l}"));
        match i {
            Instruction::Add { left, right } => {
                if let Some(l) = &mut top_tree.left {
                    l.insert(left)
                } else {
                    top_tree.left = Some(left.into());
                }
                if let Some(r) = &mut top_tree.right {
                    r.insert(right)
                } else {
                    top_tree.right = Some(right.into());
                }
            }
            Instruction::Swap(id) => {
                let ptrs = top_tree.find_mut(id);
                assert_eq!(ptrs.len(), 2);
                let mut iter = ptrs.into_iter();
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();
                if PART == 3 {
                    std::mem::swap(a,b);
                } else {
                    std::mem::swap(&mut a.label, &mut b.label);
                    std::mem::swap(&mut a.rank, &mut b.rank);
                }
            }
        }
    }
    let l = top_tree.left.unwrap().read_widest_rank();
    let r = top_tree.right.unwrap().read_widest_rank();
    l + &r
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1A: &str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
    const EG1B: &str = "ADD id=1 left=[160,E] right=[175,S]
ADD id=2 left=[140,W] right=[224,D]
ADD id=3 left=[122,U] right=[203,F]
ADD id=4 left=[204,N] right=[114,G]
ADD id=5 left=[136,V] right=[256,H]
ADD id=6 left=[147,G] right=[192,O]
ADD id=7 left=[232,I] right=[154,K]
ADD id=8 left=[118,E] right=[125,Y]
ADD id=9 left=[102,A] right=[210,D]
ADD id=10 left=[183,Q] right=[254,E]
ADD id=11 left=[146,E] right=[148,C]
ADD id=12 left=[173,Y] right=[299,S]
ADD id=13 left=[190,B] right=[277,B]
ADD id=14 left=[124,T] right=[142,N]
ADD id=15 left=[153,R] right=[133,M]
ADD id=16 left=[252,D] right=[276,M]
ADD id=17 left=[258,I] right=[245,P]
ADD id=18 left=[117,O] right=[283,!]
ADD id=19 left=[212,O] right=[127,R]
ADD id=20 left=[278,A] right=[169,C]";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1A), "CFGNLK");
        assert_eq!(solve::<1>(EG1B), "EVERYBODYCODES");
    }
    const EG2 : &str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG2), "MGFLNK");
    }

    const EG3A : &str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2";
    const EG3B : &str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5";
    #[test]
    fn p3_example_a() {
        assert_eq!(solve::<3>(EG3A), "DJMGL");
    }
    #[test]
    fn p3_example_b() {
        assert_eq!(solve::<3>(EG3B), "DJCGL");
    }
    
    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), "QUACK!JGVJZYXZ");
        assert_eq!(solve::<2>(P2_INPUT), "QUACK!PXBZJGZPBWTGJS");
        assert_eq!(solve::<3>(P3_INPUT), "QUACK!PPHPGWYYZSSBXVGLPGRBWNVVMJVH");
    }
}
