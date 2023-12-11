use nom::character::complete::space1;
pub use util::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Card {
    pub label: u8,
    pub joker_mode: bool,
}

impl Card {
    fn value(&self) -> u8 {
        match self.label {
            b'2'..=b'9' => self.label - b'2' + 1,
            b'T' => 9,
            b'J' => {
                if self.joker_mode {
                    0
                } else {
                    10
                }
            }
            b'Q' => 11,
            b'K' => 12,
            b'A' => 13,
            _ => panic!(""),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = String::from_utf8(vec![self.label]).unwrap();
        f.debug_struct("Card").field("l", &l).finish()
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, Eq)]
pub struct Hand {
    pub rank: u8,
    pub cards: [Card; 5],
    pub bid: u32,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: u32) -> Self {
        let rank = Hand::rank(&cards);
        Self { cards, bid, rank }
    }

    fn rank(cards: &[Card; 5]) -> u8 {
        let uniq: Vec<Card> = cards.iter().cloned().unique().collect();
        if uniq.len() == 1 {
            return 7;
        }
        let first_reps = cards.iter().filter(|c| **c == uniq[0]).count();
        if uniq.len() == 2 && [4, 1].contains(&first_reps) {
            return 6;
        }
        let second_reps = cards.iter().filter(|c| **c == uniq[1]).count();
        if uniq.len() == 2 && [2, 3].contains(&first_reps) && [2, 3].contains(&second_reps) {
            return 5;
        }
        let third_reps = cards.iter().filter(|c| **c == uniq[2]).count();
        if uniq.len() == 3 && first_reps == 3 || second_reps == 3 || third_reps == 3 {
            return 4;
        }
        let third_reps = cards.iter().filter(|c| **c == uniq[2]).count();
        let reps = (first_reps, second_reps, third_reps);
        if [(1, 2, 2), (2, 1, 2), (2, 2, 1)].contains(&reps) {
            return 3;
        }
        if uniq.len() == 4 {
            return 2;
        }
        1
    }

    pub fn set_jokered(&mut self) {
        for card in &mut self.cards {
            card.joker_mode = true
        }

        let num_jokers = self.cards.iter().filter(|c| c.label == b'J').count();
        let options = b"23456789TQKA".repeat(num_jokers);
        for replacements in options.iter().combinations(num_jokers) {
            let mut replaced = 0;
            let mut cards = self.cards.clone();
            for c in cards.iter_mut() {
                if c.label == b'J' {
                    c.label = *replacements[replaced];
                    replaced += 1;
                }
            }
            assert_eq!(replaced, num_jokers);
            self.rank = self.rank.max(Hand::rank(&cards));
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    all_consuming(terminated(separated_list1(newline, hand), multispace0))(input)
}

pub fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(many1(card), space1, nom_u32)(input)?;
    let cards = cards.try_into().unwrap();

    Ok((input, Hand::new(cards, bid)))
}

pub fn card(input: &str) -> IResult<&str, Card> {
    let (input, label) = one_of("234566789TJQKA")(input)?;

    let label = label.to_string().as_bytes()[0];

    Ok((
        input,
        Card {
            label,
            joker_mode: false,
        },
    ))
}
