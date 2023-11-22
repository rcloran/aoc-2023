use anyhow::{Context, Result};
use ::nom::character::complete::{i64 as nom_i64, u64 as nom_u64};
use ::nom::IResult;

pub trait Nom2Anyhow<I, O> {
    fn n2a(self) -> Result<(I, O)>;
}

impl<I, O> Nom2Anyhow<I, O> for Result<(I, O), nom::Err<nom::error::Error<&str>>> {
    fn n2a(self) -> Result<(I, O)> {
        self.map_err(|e| e.to_owned()).context("Parse failed")
    }
}

pub fn nom_isize(input: &str) -> IResult<&str, isize> {
    let (input, n) = nom_i64(input)?;
    Ok((input, n as isize))
}

pub fn nom_usize(input: &str) -> IResult<&str, usize> {
    let (input, n) = nom_u64(input)?;
    Ok((input, n as usize))
}
