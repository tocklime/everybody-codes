use std::collections::HashMap;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q06_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q06_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q06_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> String {
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let (head, rest) = l.split_once(":").unwrap();
        tree.insert(
            head.to_string(),
            rest.split(",")
                .filter(|x| *x != "BUG" && *x != "ANT")
                .map(|x| x.to_string())
                .collect(),
        );
    }
    let mut paths: HashMap<usize, Vec<Vec<&str>>> = HashMap::new();
    let mut to_explore = vec![("RR", vec![])];
    while let Some((node, path)) = to_explore.pop() {
        let mut new_path = path.clone();
        new_path.push(node);
        if node == "@" {
            paths.entry(new_path.len()).or_default().push(new_path);
        } else if let Some(children) = tree.get(node) {
            to_explore.extend(children.iter().map(|x| {
                if new_path.contains(&&x[..]) {
                    panic!("path {:?} already has {}", new_path, x);
                } else {
                    (&x[..], new_path.clone())
                }
            }));
        }
    }
    let best = &paths.values().find(|x| x.len() == 1).unwrap()[0];
    match PART {
        1 => best.join(""),
        2 | 3 => best
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect(),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@";
    #[test]
    fn p1_example() {
        assert_eq!(&solve::<1>(EG1), "RRB@");
    }
}
