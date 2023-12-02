pub use util::prelude::*;

#[derive(Debug)]
pub struct Game<'a> {
    pub id: usize,
    pub rounds: Vec<Vec<(usize, &'a str)>>,
}

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    all_consuming(terminated(separated_list1(newline, game), multispace0))(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), nom_usize, tag(": "))(input)?;
    let (input, rounds) = separated_list1(tag("; "), round)(input)?;

    Ok((input, Game { id, rounds }))
}

pub fn round(input: &str) -> IResult<&str, Vec<(usize, &str)>> {
    separated_list1(tag(", "), separated_pair(nom_usize, tag(" "), alpha1))(input)
}
