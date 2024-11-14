use std::{path::PathBuf, process::Command, str::FromStr};

use itertools::Itertools;

fn main() {
    let src = PathBuf::from_str("src/bin").unwrap();
    let mut x = std::fs::read_dir(&src).unwrap();
    let (_, best) = x
        .fold_ok(None, |acc, e| {
            if e.file_name() != "most_recent.rs" {
                let modified = e.metadata().unwrap().modified().unwrap();
                match acc {
                    Some((dt, _)) if dt > modified => {}
                    _ => {
                        return Some((modified, e.file_name()));
                    }
                }
            }
            return acc;
        })
        .unwrap()
        .unwrap();
    dbg!(&best);

    let target_str = best.to_string_lossy();
    let target = target_str.split(".").next().unwrap();
    let test = Command::new("cargo")
        .args(["test", "--bin", target]).spawn().unwrap().wait().unwrap();
    println!("Tests status: {test:?}");
    let res = Command::new("cargo")
        .args(["run", "-r", "--bin", target])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    println!("Exit status: {res:?}");
}
