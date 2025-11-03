use std::{fmt::Debug, str::FromStr};

use nom::Parser;

#[allow(clippy::missing_errors_doc)]
pub fn try_parse_many<T, E>(input: &str, sep: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
{
    input.split(sep).map(str::parse).collect()
}
#[must_use]
pub fn parse_input_from_str_sep_by<T>(input: &str, sep: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input_from_str_sep_by(input, sep, |x| {
        x.parse().expect("Could not parse item from input")
    })
}

pub fn input_from_str_sep_by<'a, T, F>(input: &'a str, sep: &str, f: F) -> Vec<T>
where
    F: Fn(&'a str) -> T + 'a,
{
    input.trim().split(sep).map(|x| f(x.trim())).collect()
}

pub fn get_matches_from_str<'a, T, P>(mut input: &'a str, mut parser: P) -> Vec<T> 
where P : Parser<&'a str, Output = T, Error = ()> 
{
    let mut ans = Vec::new();
    while !input.is_empty() {
        match parser.parse(input) {
            Ok((rest, t)) => {
                ans.push(t);
                input = rest;
            }
            Err(_) => {
                input = &input[1..];
            }
        }
    }
    ans 

}