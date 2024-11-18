use std::collections::HashSet;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q02_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q02_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q02_p3.txt");

fn solve(str: &str) -> usize {
    let first_newline = str.find("\n").unwrap();
    let (words, rest) = str.split_at(first_newline);
    let words: HashSet<&str> = words.split(":").nth(1).unwrap().split(",").collect();
    words.iter().map(|w| rest.matches(w).count()).sum()
}
fn solve2(str: &str) -> usize {
    let first_newline = str.find("\n").unwrap();
    let (words, rest) = str.split_at(first_newline);
    let words: HashSet<String> = words
        .split(":")
        .nth(1)
        .unwrap()
        .split(",")
        .flat_map(|c| [c.to_string(), c.chars().rev().collect()])
        .collect();
    let mut highlight_char_ixs = HashSet::new();
    for i in 0..rest.len() {
        for w in &words {
            if rest[i..].starts_with(w) {
                highlight_char_ixs.extend(i..i + w.len());
            }
        }
    }
    highlight_char_ixs.len()
}

fn solve3(str: &str) -> usize {
    let first_newline = str.find("\n").unwrap();
    let (words, rest) = str.split_at(first_newline);
    let words: HashSet<Vec<u8>> = words
        .split(":")
        .nth(1)
        .unwrap()
        .split(",")
        .flat_map(|c| {
            [
                c.as_bytes().to_vec(),
                c.as_bytes().iter().cloned().rev().collect::<Vec<u8>>(),
            ]
        })
        .collect();
    let grid = rest
        .trim()
        .lines()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut highlight_char_ixs = HashSet::new();
    for r in 0..grid.len() {
        let row = &grid[r];
        for c in 0..row.len() {
            for w in &words {
                //horiz
                let cand = row.iter().cycle().skip(c).take(w.len());
                if cand.eq(w.iter()) {
                    for ix in 0..w.len() {
                        highlight_char_ixs.insert((r, (c + ix) % row.len()));
                    }
                }
                //vert
                let cand = grid[r..].iter().map(|x| &x[c]).take(w.len());
                if cand.eq(w.iter()) {
                    for ix in 0..w.len() {
                        highlight_char_ixs.insert((r + ix, c));
                    }
                }
            }
        }
    }
    // print_grid(&grid, &highlight_char_ixs);
    highlight_char_ixs.len()
}
#[allow(dead_code)]
fn print_grid(grid: &[Vec<u8>], highlighted: &HashSet<(usize, usize)>) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &byte) in row.iter().enumerate() {
            let ch: char = char::from(byte);
            if highlighted.contains(&(r, c)) {
                print!("{}", ansi_term::Color::Red.paint(ch.to_string()));
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
}

fn main() {
    println!("P1: {}", solve(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p1_example() {
        let a = solve(
            "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE",
        );
        assert_eq!(a, 4);
    }
    #[test]
    fn p2_example() {
        let a = solve2(
            "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END",
        );
        assert_eq!(a, 37);
    }
    #[test]
    fn p3_example() {
        let a = solve3(
            "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL",
        );
        assert_eq!(a, 10);
    }
}
