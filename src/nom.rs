use nom::{character::complete::multispace0, error::ParseError, sequence::delimited, Parser};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

pub type NomError<'a> = nom_supreme::error::ErrorTree<&'a str>;
pub type IResult<'a, T> = nom::IResult<&'a str, T, NomError<'a>>;

