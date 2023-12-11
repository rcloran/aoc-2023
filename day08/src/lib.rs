pub use util::prelude::*;

#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

pub fn parse(input: &str) -> IResult<&str, (Vec<char>, Vec<Node>)> {
    separated_pair(
        many1(one_of("RL")),
        tag("\n\n"),
        separated_list1(newline, node),
    )(input)
}

pub fn node(input: &str) -> IResult<&str, Node> {
    let (input, (name, (left, right))) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)?;

    Ok((input, Node { name, left, right }))
}
