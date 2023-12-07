use std::{collections::HashMap, error::Error, fs, path::PathBuf};

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(s: &char) -> Result<Self, &'static str> {
        // Replace jack with joker later
        match s {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err("Invalid card char"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn new(cards: &[Card; 5]) -> Result<Self, &'static str> {
        let mut card_counts = HashMap::new();

        for card in cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let joker_count = card_counts.remove(&Card::Joker).unwrap_or(0);

        let mut counts;
        if joker_count == 5 {
            counts = vec![5]
        } else {
            counts = card_counts.into_values().collect::<Vec<_>>();
            counts.sort();
            *counts.last_mut().unwrap() += joker_count;
        }

        match counts.as_slice() {
            [5] => Ok(Self::FiveKind),
            [1, 4] => Ok(Self::FourKind),
            [2, 3] => Ok(Self::FullHouse),
            [1, 1, 3] => Ok(Self::ThreeKind),
            [1, 2, 2] => Ok(Self::TwoPair),
            [1, 1, 1, 2] => Ok(Self::OnePair),
            [1, 1, 1, 1, 1] => Ok(Self::HighCard),
            _ => Err("Invalid hand type"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: i64,
}

impl Hand {
    fn new(s: &str, part_two: bool) -> Result<Self, &'static str> {
        let (card_str, bid_str) = s.split_once(' ').unwrap();

        let bid = bid_str.parse::<i64>().unwrap();

        let cards = card_str
            .chars()
            .map(|c| Card::from_char(&c).unwrap())
            .map(|c| {
                // Replace J with joker.
                if part_two && c == Card::J {
                    Card::Joker
                } else {
                    c
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let hand_type = HandType::new(&cards).unwrap();
        Ok(Self {
            hand_type: hand_type,
            cards: cards,
            bid: bid,
        })
    }
}

pub fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let mut hands = puzzle_input
        .lines()
        .map(|l| Hand::new(l, part_two).unwrap())
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as i64 * h.bid)
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_greater_than() {
        let queen = Card::from_char(&'Q').unwrap();
        let three = Card::from_char(&'3').unwrap();

        assert!(queen > three)
    }

    #[test]
    fn test_card_equal() {
        let six_1 = Card::from_char(&'6').unwrap();
        let six_2 = Card::from_char(&'6').unwrap();

        assert!(six_1 == six_2)
    }

    #[test]
    fn test_card_less_than() {
        let two = Card::from_char(&'2').unwrap();
        let ace = Card::from_char(&'A').unwrap();

        assert!(two < ace);
    }

    #[test]
    fn test_hand_ordering_on_type() {
        let hand_1 = Hand::new("32T3K 765", false).unwrap();

        let hand_2 = Hand::new("T55J5 684", false).unwrap();

        assert!(hand_2 > hand_1);
    }

    #[test]
    fn test_hand_ordering_on_cards() {
        let hand_1 = Hand::new("77788 1", false).unwrap();
        let hand_2 = Hand::new("77888 1", false).unwrap();

        assert!(hand_2 > hand_1);
    }

    #[test]
    fn test_joker_hand_type() {
        let hand = Hand::new("KTJJT 1", true).unwrap();
        assert!(hand.hand_type == HandType::FourKind);
    }
}
