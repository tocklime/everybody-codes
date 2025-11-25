use everybody_codes::{cartesian::Point, grid2d::Grid2d};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q17_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q17_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q17_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let g= Grid2d::from_str(input, |x| x);
    let v = g.find_elem(&'@').unwrap();
    let f = |r: usize, c: Point<usize>| -> bool {
        let x_diff = v.x.abs_diff(c.x);
        let y_diff = v.y.abs_diff(c.y);
        x_diff * x_diff + y_diff * y_diff <= r * r
    };
    let r_destruction = |r| -> usize {
        g.indexed_iter().filter(|(p,_)| f(r,*p)).map(|(_, &x)| if x == '@'  {0} else { (x as u8) - b'0'} as usize).sum()
    };
    match PART {
        1 => {
            r_destruction(10)
        }
        2 => {
            let destrs = (0..).map(|r| r_destruction(r)).tuple_windows().map(|(a,b)| b-a ).take_while(|x| *x > 0).collect::<Vec<usize>>();
            let (a,b) = destrs.iter().zip(1..).max().unwrap();
            a*b

        }
        3 => {
            for r in 0.. {
                let max_time = (r+1) * 30;
                let start = g.find_elem(&'S').unwrap();
                let d = pathfinding::directed::astar::astar(&(start,0), |(p, stage)| {
                    g.neighbours(*p).into_iter().filter(|n| {
                        !f(r,*n)
                    }).map(|n| {
                        let new_stage = match stage {
                            0 => {
                                n.y == v.y && n.x < v.x
                            },
                            1 => {
                                n.x == v.x && n.y > v.y
                            }
                            2 => {
                                n.y == v.y && n.x > v.x
                            }
                            _ => false
                        };
                        let c = g[n];
                        let cost = if c == 'S' { 0 } else { (c as u8) - b'0' } as usize;
                        ((n, stage+if new_stage {1 } else {0}),cost) 
                    }).collect_vec()
                },|&(p, stage)| {
                    match stage {
                        0 => {
                            v.y-p.y + p.x.saturating_sub(v.x)
                        }
                        1 => {
                            v.x-p.x + v.y.saturating_sub(p.y)
                        }
                        2 => {
                            p.y-v.y + v.x.saturating_sub(p.x)
                        }
                        3 => {
                            p.x.abs_diff(start.x) + p.y.abs_diff(start.y)
                        }
                        _ => unreachable!()
                    }

                }, |&(p, stage)| stage == 3 && p == start);
                if let Some((path, cost)) = d {
                    if cost < max_time {
                        let mut d = g.map(|_,c| c.to_string());
                        for &(p, _) in &path {
                            d[p] = format!("{}",ansiterm::Color::Green.bold().paint(d[p].clone()));
                        }
                        for (p,c) in d.indexed_iter_mut() {
                            if f(r,p) {
                                *c = ".".to_string();
                            }
                        }
                        println!("Got path of cost {cost} with radius {r}:\n{d}");
                        return cost * r;
                    }
                    println!("Solve for r={r} takes {cost} (>{max_time})");
                }
            }
            unreachable!()
        }
        _ => todo!()
    }

}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 1573);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>("4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449"), 1090);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>("2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464"), 592);
    }
    #[test]
    fn p3_example3() {
        assert_eq!(solve::<3>("5441525241225111112253553251553
133522122534119S911411222155114
3445445533355599933443455544333
3345333555434334535435433335533
5353333345335554434535533555354
3533533435355443543433453355553
3553353435335554334453355435433
5435355533533355533535335345335
4353545353545354555534334453353
4454543553533544443353355553453
5334554534533355333355543533454
4433333345445354553533554555533
5554454343455334355445533453453
4435554534445553335434455334353
3533435453433535345355533545555
534433533533535@353533355553345
4453545555435334544453344455554
4353333535535354535353353535355
4345444453554554535355345343354
3534544535533355333333445433555
3535333335335334333534553543535
5433355333553344355555344553435
5355535355535334555435534555344
3355433335553553535334544544333
3554333535553335343555345553535
3554433545353554334554345343343
5533353435533535333355343333555
5355555353355553535354333535355
4344534353535455333455353335333
5444333535533453535335454535553
3534343355355355553543545553345"), 3180);
    }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}