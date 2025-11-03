use std::{
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use itertools::Itertools;

fn main() {
    let src = PathBuf::from_str("src/bin").unwrap();
    let x = std::fs::read_dir(&src).unwrap();
    let to_do = x
        .filter_map(|e| {
            let file_name = e.ok()?.file_name().into_string().unwrap();
            if file_name.starts_with("e") && file_name.ends_with(".rs") {
                Some(file_name)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
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

    for target_str in to_do.iter().sorted() {
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

}
