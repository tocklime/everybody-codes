use std::{
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use itertools::Itertools;

fn main() {
    let src = PathBuf::from_str("src/bin").unwrap();
    let mut x = std::fs::read_dir(&src).unwrap();
    let (_, best) = x
        .fold_ok(None, |acc, e| {
            if e.file_name() != "most_recent.rs" && e.file_name() != "template.txt" {
                let modified = e.metadata().unwrap().modified().unwrap();
                match acc {
                    Some((dt, _)) if dt > modified => {}
                    _ => {
                        return Some((modified, e.file_name()));
                    }
                }
            }
            acc
        })
        .unwrap()
        .unwrap();
    let download_dir = platform_dirs::UserDirs::new().unwrap().download_dir;
    let target_dir = PathBuf::from_str("inputs").unwrap();

    let everybody_codes_text_files = std::fs::read_dir(&download_dir)
        .unwrap()
        .filter_map(|x| x.ok().and_then(|x| x.file_name().into_string().ok()))
        .filter(|f| f.starts_with("everybody_codes") && f.ends_with(".txt") && !f.contains("("));
    for f in everybody_codes_text_files {
        let source = download_dir.join(&f);
        let target = target_dir.join(&f);
        if !target.exists() {
            println!("Copying {:?} to {:?}", &source, &target);
            std::fs::copy(&source, &target).unwrap();
        }
    }

    let target_str = best.to_string_lossy();
    let target = target_str.split(".").next().unwrap();
    let mut test = Command::new("cargo")
        .args(["test", "--bin", target])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let res = test.wait().unwrap();
    if !res.success() {
        let mut stdout = String::new();
        test.stdout.unwrap().read_to_string(&mut stdout).unwrap();
        println!("{}", stdout);
    } else {
        let res = Command::new("cargo")
            .args(["run", "-r", "--bin", target])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        println!("Exit status: {res:?}");
    }
}
