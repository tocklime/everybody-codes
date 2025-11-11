use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q07_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q07_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q07_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn finish_name(
    seen: &mut HashSet<String>,
    rules: &HashMap<char, Vec<char>>,
    name: String
) -> HashSet<String> {
    let mut ans = HashSet::new();
    if !seen.insert(name.clone()) {
        return HashSet::new()
    }
    if name.len() >= 7 {
        ans.insert(name.clone());
    }
    match rules.get(&name.chars().last().unwrap()) {
        None => {},
        Some(r) => {
            if name.len() < 11 {
                for opt in r {
                    ans.extend(finish_name(seen, rules, format!("{name}{opt}")).into_iter());
                }
            }
        }
    };
    ans
}
fn solve<const PART: usize>(input: &str) -> String {
    let (names, rules) = input.split_once("\n\n").unwrap();
    let names = names.split(",").collect_vec();
    let rules: HashMap<char, Vec<char>> = rules
        .lines()
        .map(|l| {
            let (i, o) = l.split_once(" > ").unwrap();
            let o = o
                .split(",")
                .map(|s| s.chars().next().unwrap())
                .collect_vec();
            (i.chars().next().unwrap(), o)
        })
        .collect();
    let match_name = |n: &&&str| {
        let mut ch = n.chars();
        let mut this_char = ch.next().unwrap();
        while let Some(r) = rules.get(&this_char) {
            let next_char = ch.next();
            match next_char {
                None => {
                    return true;
                }
                Some(n) => {
                    this_char = n;
                }
            }

            if !r.contains(&this_char) {
                return false;
            }
        }
        return true;
    };
    match PART {
        1 => names.iter().find(match_name).unwrap().to_string(),
        2 => names
            .iter()
            .positions(|x| match_name(&x))
            .map(|x| x+1)
            .sum::<usize>()
            .to_string(),
        3 => {
            // let mut memo = HashMap::new();
            let mut seen = HashSet::new();
            names
                .iter()
                .filter_map(|n| {
                    if !match_name(&n) {
                        return None;
                    }
                    let names = finish_name(&mut seen, &rules,n.to_string());
                    Some(names)
                })
                .flatten()
                .collect::<HashSet<String>>()
                .into_iter()
                .count()
                .to_string()
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), "Oroneth");
    }
    #[test]
    fn p2_example() {
        const EG: &str = "Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";
        assert_eq!(solve::<2>(EG), "23");
    }
    #[test]
    fn p3_example() {
        const EG1: &str = "Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t";
        const EG2: &str = "Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";
        assert_eq!(solve::<3>(EG1), "25");
        assert_eq!(solve::<3>(EG2), "1154");
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), "Nyendris");
        assert_eq!(solve::<2>(P2_INPUT), "2521");
        assert_eq!(solve::<3>(P3_INPUT), "2108001");
    }
}

//TODO: this one is currently pretty slow (takes several seconds for p3). Needs speeding up.
// has the feel of a problem that will do well to be sped up with dynamic programming, but has the annoying 'only the starts that are listed' condition.
// and also, the 'remove the duplicates made from different stems' problem.
// I guess the latter could be fixed by pre-filtering the list so we only keep the shortest valid stems, then use those to look into the dp table. 
// Or do it on demand with a memo.