use num::integer::sqrt;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q08_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q08_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q08_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let n: usize = input.parse().unwrap();
    match PART {
        1 => {
            let sr = sqrt(n)+1;
            ((sr * sr) - n) * (2 * sr - 1)
        }
        2 => {
            P2::new(20240000, n as i64, 1111).solve().try_into().unwrap()
        }
        3 => P3::new(n as u64, 10, 202400000).solve().try_into().unwrap(),
        _ => unimplemented!(),
    }
}
#[derive(Debug)]
struct P2 {
    thickness: i64,
    layer_width: i64,
    blocks_remaining: i64,
    priests: i64,
    acolytes: i64,
}

impl P2 {
    fn new(blocks: i64, priests: i64, acolytes: i64) -> Self {
        Self {
            thickness: 1,
            layer_width: 1,
            blocks_remaining: blocks,
            priests,
            acolytes,
        }
    }
    fn next_thickness(&self) -> i64 {
        (self.thickness * self.priests) % self.acolytes
    }
    fn grow(&mut self) {
        let thickness = self.thickness;
        let width = self.layer_width;
        self.thickness = self.next_thickness();
        self.layer_width += 2;
        let blocks_for_this_layer = width * thickness;
        self.blocks_remaining -= blocks_for_this_layer;
    }
    fn solve(mut self) -> i64 {
        while self.blocks_remaining > 0 {
            self.grow();
        }
        (-self.blocks_remaining) * (self.layer_width - 2)
    }
}

#[derive(Clone, Debug)]
struct P3 {
    high_priests: u64,
    high_priest_acolytes: u64,
    blocks_available: u64,
    thickness: u64,
    column_heights: Vec<u64>,
}
impl P3 {
    fn new(high_priests: u64, high_priest_acolytes: u64, blocks_available: u64) -> Self {
        Self {
            high_priests,
            high_priest_acolytes,
            blocks_available,
            thickness: 1,
            column_heights: vec![1],
        }
    }
    fn grow(&mut self) {
        let current_thickness = self.thickness;
        let next_thickness = (current_thickness * self.high_priests) % self.high_priest_acolytes
            + self.high_priest_acolytes;
        for x in &mut self.column_heights {
            *x += next_thickness;
        }
        self.column_heights.push(next_thickness);
        self.thickness = next_thickness;
    }
    fn count_blocks(&self) -> u64 {
        let width = u64::try_from(self.column_heights.len() * 2 - 1).unwrap();
        let mut last_height = 0;
        let mut ans = 0;
        let mut last_added = 0;
        for &h in self.column_heights.iter().rev() {
            let to_remove = (self.high_priests * width * h) % self.high_priest_acolytes;
            let to_remove = to_remove.min(last_height).min(h - 1);
            last_added = h - to_remove;
            ans += 2 * last_added;
            last_height = h - 1;
        }
        ans - last_added
    }
    fn solve(mut self) -> u64 {
        while self.count_blocks() < self.blocks_available {
            self.grow();
        }
        self.count_blocks() - self.blocks_available
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>("13"), 21);
        assert_eq!(solve::<1>("3"), 3);
        assert_eq!(solve::<1>("6"), 15);
    }
    #[test]
    fn p2_example() {
        assert_eq!(P2::new(50, 3, 5).solve(), 27)
    }
    #[test]
    fn p3_example() {
        assert_eq!(P3::new(2, 5, 160).solve(), 2)
    }
}
