const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q01_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q01_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q01_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> String {
    let (names, instrs) = input.split_once("\n\n").unwrap();
    let mut names = names.split(",").collect::<Vec<_>>();
    let names_len = names.len();
    let mut pos = 0usize;
    for i in instrs.split(",") {
        let (c, rest) = i.split_at(1);
        let n = rest.parse::<usize>().unwrap();
        match c {
            "R" => match PART {
                1 => {
                    pos = (pos + n).clamp(0, names.len() - 1);
                }
                2 => {
                    pos = (n + pos) % names_len;
                }
                3 => {
                    names.swap(0, n % names_len);
                }
                _ => unreachable!(),
            },
            "L" => {
                match PART {
                    1 => {
                        pos = pos.saturating_sub(n);
                    }
                    2 => {
                        if pos < n {
                            pos = (pos + names.len() - n) % names.len();
                        } else {
                            pos -= n;
                        }
                    }
                    3 => {
                        names.swap(0, (names_len - (n % names_len)) % names_len);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
    }
    names[pos].to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), "Fyrryn");
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG1), "Elarzris");
    }
    #[test]
    fn p3_example() {
        const EG: &str = "Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3";
        assert_eq!(solve::<3>(EG), "Drakzyph");
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), "Zynloris");
        assert_eq!(solve::<2>(P2_INPUT), "Vornulrix");
        assert_eq!(solve::<3>(P3_INPUT), "Kalirin");
    }
}
