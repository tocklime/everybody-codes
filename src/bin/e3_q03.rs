use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, newline, u32},
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q03_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q03_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q03_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
#[derive(Debug, Clone)]
struct Connector {
    color: String,
    shape: String,
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Match {
    Weak,
    Strong,
}
impl Connector {
    fn matches(&self, other: &Self) -> Option<Match> {
        match (self.color == other.color, self.shape == other.shape) {
            (true, true) => Some(Match::Strong),
            (false, false) => None,
            _ => Some(Match::Weak),
        }
    }
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = Err<'a>> {
        (alpha1::<&'a str, _>, tag(" "), alpha1).map(|(color, _, shape)| Self {
            color: color.to_string(),
            shape: shape.to_string(),
        })
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node {
    id: usize,
    plug: Connector,
    left_socket: Connector,
    right_socket: Connector,
    left: Option<(Match, usize)>,
    right: Option<(Match, usize)>,
    data: String,
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Position {
    Left,
    Right,
}
type Err<'a> = nom::error::Error<&'a str>;
impl Node {
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = Err<'a>> {
        // id=2, plug=GREEN CIRCLE, leftSocket=BLUE HEXAGON, rightSocket=BLUE CIRCLE, data=?
        (
            tag("id="),
            u32::<&'a str, _>,
            tag(", plug="),
            Connector::parse(),
            tag(", leftSocket="),
            Connector::parse(),
            tag(", rightSocket="),
            Connector::parse(),
            tag(", data="),
            take_until("\n"),
        )
            .map(
                move |(_, id, _, plug, _, left_socket, _, right_socket, _, data)| Self {
                    id: (id - 1) as usize,
                    plug,
                    left_socket,
                    right_socket,
                    left: None,
                    right: None,
                    data: data.to_string(),
                },
            )
    }

    fn try_place(&mut self, other: &Self, pos: Position) -> Option<Option<usize>> {
        match pos {
            Position::Left => {
                let left_match = self.left_socket.matches(&other.plug);
                match (left_match, self.left) {
                    (_, Some((Match::Strong, _))) => {}
                    (Some(Match::Strong), Some((_, old))) => {
                        self.left = Some((Match::Strong, other.id));
                        return Some(Some(old));
                    }
                    (Some(m), None) => {
                        self.left = Some((m, other.id));
                        return Some(None);
                    }
                    _ => {}
                }
            }
            Position::Right => {
                let right_match = self.right_socket.matches(&other.plug);
                match (right_match, self.right) {
                    (_, Some((Match::Strong, _))) => {}
                    (Some(Match::Strong), Some((_, old))) => {
                        self.right = Some((Match::Strong, other.id));
                        return Some(Some(old));
                    }
                    (Some(m), None) => {
                        self.right = Some((m, other.id));
                        return Some(None);
                    }
                    _ => {}
                }
            }
        }
        None
    }
    fn fits_left(&self, other: &Self, match_level: Match) -> bool {
        if self.left.is_none() {
            self.left_socket
                .matches(&other.plug)
                .map(|x| x >= match_level)
                .unwrap_or_default()
        } else {
            false
        }
    }
    fn fits_right(&self, other: &Self, match_level: Match) -> bool {
        if self.right.is_none() {
            self.right_socket
                .matches(&other.plug)
                .map(|x| x >= match_level)
                .unwrap_or_default()
        } else {
            false
        }
    }
    fn fits(&self, other: &Self, match_level: Match) -> bool {
        self.fits_left(other, match_level) || self.fits_right(other, match_level)
    }
}
#[derive(Debug)]
struct AllNodes(Vec<Node>);
enum Dir {
    PreLeft,
    PreRight,
    Next,
}
impl Dir {
    fn from_pos(p: Position) -> Self {
        match p {
            Position::Left => Self::PreRight,
            Position::Right => Self::Next,
        }

    }
}
impl AllNodes {
    fn route_around(&self) -> VecDeque<(usize, Position)> {
        let init = (0, Position::Left);
        let mut pos = vec![init];
        let mut ans = VecDeque::new();
        while let Some((n, p)) = pos.pop() {
            ans.push_back((n, p));
            match p {
                Position::Left => {
                    pos.push((n, Position::Right));
                    if let Some((_, l)) = self.0[n].left {
                        pos.push((l, Position::Left));
                    }
                }
                Position::Right => {
                    if let Some((_, r)) = self.0[n].right {
                        pos.push((r, Position::Left))
                    }
                }
            }
        }
        ans
    }
    fn walk<'a>(&'a self) -> Walker<'a> {
        let mut init_stack = vec![];
        let mut curr = 0;
        while let Some(n) = self.0[curr].left {
            init_stack.push(curr);
            curr = n.1;
        }
        init_stack.push(curr);
        Walker {
            all_nodes: self,
            stack: init_stack,
        }
    }
}
struct Walker<'a> {
    all_nodes: &'a AllNodes,
    stack: Vec<usize>,
}
impl<'a> Iterator for Walker<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.stack.pop() {
            let b = &self.all_nodes.0[n];
            if let Some((_, right)) = b.right {
                let mut curr = right;
                while let Some((_, n)) = self.all_nodes.0[curr].left {
                    self.stack.push(curr);
                    curr = n;
                }
                self.stack.push(curr);
            }
            Some(n)
        } else {
            None
        }
    }
}

fn solve<const PART: usize>(input: &str) -> usize {
    let match_level = match PART {
        1 => Match::Strong,
        2 => Match::Weak,
        _ => Match::Weak,
    };
    let input = format!("{input}\n");
    let (_, plugs) = nom::multi::separated_list1(newline, Node::parse())
        .parse(&input)
        .unwrap();
    let mut all_nodes = AllNodes(plugs);
    for next_node in 1..all_nodes.0.len() {
        if PART == 3 {
            let mut to_place = Some((0, next_node, Some(Position::Left)));
            while let Some((start, id, pos)) = to_place.take() {
                let n = all_nodes.0[id].clone();
                let mut nodes = all_nodes.route_around();
                match pos {
                    Some(p) =>  {
                        while nodes.front() != Some(&(start, p)) {
                            nodes.rotate_left(1);
                        }
                    }
                    None => {
                        while nodes.front() != Some(&(start, Position::Right)) {
                            nodes.rotate_left(1);
                        }
                        nodes.rotate_left(1);
                    }
                }
                // println!("walk order: {nodes:?}");
                // println!("Placing: {n:?}");
                let (replaced_pos, usurped, lr) = nodes
                    .into_iter()
                    .find_map(|(c, pos)| all_nodes.0[c].try_place(&n, pos).map(|x| (c, x, pos)))
                    .unwrap();
                if let Some(u) = usurped {
                    //now we need to place this one.
                    let next =match lr {
                        Position::Left => {
                            (replaced_pos, Some(Position::Right))
                        }
                        Position::Right => {
                            (all_nodes.0[replaced_pos].right.unwrap().1, None)
                        }
                    };
                    println!("Replaced node id {u}, will start next search from {next:?}");
                    // println!("{all_nodes:?}");
                    to_place = Some((next.0, u, next.1));
                }
            }
        } else {
            let n = all_nodes.0[next_node].clone();
            let hole = all_nodes
                .walk()
                .find(|c| all_nodes.0[*c].fits(&n, match_level))
                .unwrap();
            let to_mod = &mut all_nodes.0[hole];
            if to_mod.fits_left(&n, match_level) {
                to_mod.left = Some((Match::Strong, next_node));
            } else {
                assert!(to_mod.fits_right(&n, match_level));
                to_mod.right = Some((Match::Strong, next_node));
            }
        }
    }
    // println!("{:?}", all_nodes.walk().collect_vec());
    all_nodes.walk().zip(1..).map(|(n, m)| (n + 1) * m).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str =
        "id=1, plug=BLUE HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=BLUE PENTAGON, data=?
id=2, plug=GREEN CIRCLE, leftSocket=BLUE HEXAGON, rightSocket=BLUE CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=BLUE CIRCLE, data=?
id=4, plug=BLUE CIRCLE, leftSocket=RED HEXAGON, rightSocket=BLUE HEXAGON, data=?
id=5, plug=RED HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=RED HEXAGON, data=?";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 43);
    }
    const EG2: &str =
        "id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?";
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG2), 50);
    }
    const EG3: &str =
        "id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?";
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 38);
    }
    const EG3A: &str =
        "id=1, plug=RED TRIANGLE, leftSocket=BLUE TRIANGLE, rightSocket=GREEN TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?
id=6, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?";
    #[test]
    fn p3_example_a() {
        assert_eq!(solve::<3>(EG3A), 60);
    }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}
