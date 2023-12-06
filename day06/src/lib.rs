pub use util::prelude::*;

pub fn parse(input: &str) -> IResult<&str, Vec<usize>> {
    all_consuming(terminated(separated_list1(newline, nom_usize), multispace0))(input)
}
