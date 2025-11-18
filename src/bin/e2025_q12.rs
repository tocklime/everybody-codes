use std::collections::HashSet;

use everybody_codes::{cartesian::Point, grid2d::Grid2d};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q12_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q12_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q12_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
fn set_fire(g: &Grid2d<u8>, starts: Vec<Point<usize>>, alight: &mut Grid2d<bool>) {
    let mut fringe : HashSet<Point<usize>> = starts.into_iter().collect();
    fringe.iter().for_each(|p| alight[*p] = true);
    while !fringe.is_empty() {
        fringe = fringe.iter().flat_map(|x| g.neighbours(*x).filter(|p| g[*p] <= g[*x] && !alight[*p])).collect();
        fringe.iter().for_each(|p| alight[*p] = true);
    }

}
fn solve3(input: &str) -> usize {
    let g : Grid2d<u8> = Grid2d::from_str(input, |x| (x as u8) - b'0');
    //1.
    let best1 = g.indexes().map(|x| {
        let mut alight = Grid2d::from_elem(g.dim(), false);
        set_fire(&g, vec![x], &mut alight);
        let count = alight.iter().filter(|x| **x).count();
        (count, alight, x)
    }).max_by_key(|z| z.0).unwrap();
    println!("Best is {:?}: {}",best1.2, best1.0);
    let alight_start = best1.1;
    let best2 = g.indexes().map(|x| {
        let mut alight = alight_start.clone();
        set_fire(&g, vec![x], &mut alight);
        let count = alight.iter().filter(|x| **x).count();
        (count, alight, x)
    }).max_by_key(|x| x.0).unwrap();
    println!("Best2 is {:?}: {}",best2.2, best2.0);
    let alight_start = best2.1;
    let best3 = g.indexes().map(|x| {
        let mut alight = alight_start.clone();
        set_fire(&g, vec![x], &mut alight);
        let count = alight.iter().filter(|x| **x).count();
        (count, alight, x)
    }).max_by_key(|x| x.0).unwrap();
    println!("Best3 is {:?}: {}",best3.2, best3.0);
    let mut alight_final = Grid2d::from_elem(g.dim(), false);
    set_fire(&g, vec![
        best1.2,best2.2,best3.2
    ], &mut alight_final);
    println!("{}", alight_final.to_string_with(|x| (if *x { "#"} else {"."}).to_string()));
    alight_final.iter().filter(|x|**x).count()
}
fn solve<const PART: usize>(input: &str) -> usize {
    let g : Grid2d<u8> = Grid2d::from_str(input, |x| (x as u8) - b'0');
    let fringe = match PART {
        1 => vec![Point::new(0,0)],
        2 => vec![Point::new(0,0), g.dim() - Point::new(1,1)],
        _ => unimplemented!()
    };
    let mut alight = Grid2d::from_elem(g.dim(), false);
    set_fire(&g, fringe, &mut alight);
    alight.iter().filter(|x| **x).count()
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "989611
857782
746543
766789";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 16);
    }
    // #[test]
    // fn p2_example() {
    //     assert_eq!(solve::<2>(EG2), 0);
    // }
    #[test]
    fn p3_example() {
        const EG1: &str = "5411
3362
5235
3112";
        assert_eq!(solve3(EG1), 14);
        const EG2: &str = "41951111131882511179
32112222211508122215
31223333322105122219
31234444432147511128
91223333322176021892
60112222211166431583
04661111166111111746
01111119042122222177
41222108881233333219
71222127839122222196
56111026279711111507";
        assert_eq!(solve3(EG2), 133);
    }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}