pub use util::prelude::*;

#[derive(Debug)]
pub struct Card {
    pub winning: HashSet<usize>,
    pub have: HashSet<usize>,
}

impl Card {
    pub fn matches(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    all_consuming(terminated(separated_list1(newline, card), multispace0))(input)
}
pub fn card(input: &str) -> IResult<&str, Card> {
    let (input, parts) = tuple((
        tag("Card"),
        many1(tag(" ")),
        nom_usize,
        tag(":"),
        many1(tag(" ")),
        separated_list1(many1(tag(" ")), nom_usize),
        many1(tag(" ")),
        tag("|"),
        many1(tag(" ")),
        separated_list1(many1(tag(" ")), nom_usize),
    ))(input)?;

    let winning = HashSet::from_iter(parts.5);
    let have = HashSet::from_iter(parts.9);

    Ok((input, Card { winning, have }))
}

// This should be equivalent enough to parse() to be usable
pub fn parse_with_split(input: &str) -> Vec<Card> {
    let mut r = vec![];
    for l in input.lines() {
        let (_, nums) = l.split_once(':').unwrap();
        let (winning, have) = nums.split_once('|').unwrap();

        let winning = winning.split(' ').filter_map(|s| s.parse().ok()).collect();
        let have = have.split(' ').filter_map(|s| s.parse().ok()).collect();

        r.push(Card { winning, have })
    }
    r
}
