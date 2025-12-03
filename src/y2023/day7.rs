use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    fmt::{Display, Write},
};

use winnow::{
    ascii::{dec_int, multispace0},
    combinator::{empty, fail, repeat, separated},
    dispatch,
    prelude::*,
    token::any,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut hands = parse_input(input);

    hands.sort_by(|a, b| {
        a.hand_type()
            .cmp(&b.hand_type())
            .then_with(|| a.cards.cmp(&b.cards))
    });
    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i64 + 1) * hand.bid)
        .sum::<i64>();

    hands.sort_by(|a, b| {
        a.joker_hand_type().cmp(&b.joker_hand_type()).then_with(|| {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .map(|(c, d)| joker_cmp(c, d))
                .find(|ordering| *ordering != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    });
    // for hand in hands.iter() {
    //     println!("{hand}");
    // }
    let part2 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i64 + 1) * hand.bid)
        .sum::<i64>();

    (part1, part2)
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Tit,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        hand_type_for_cards(&self.cards)
    }

    fn joker_hand_type(&self) -> HandType {
        let joker_count = self
            .cards
            .iter()
            .filter(|card| **card == Card::Jack)
            .count();
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        let cards_without_jokers = self
            .cards
            .into_iter()
            .filter(|card| *card != Card::Jack)
            .collect::<Vec<_>>();
        let jokerless_type = hand_type_for_cards(&cards_without_jokers);

        match (jokerless_type, joker_count) {
            (_, 0) => jokerless_type,
            (HandType::HighCard, 1) => HandType::OnePair,
            (HandType::HighCard, 2) => HandType::ThreeOfAKind,
            (HandType::HighCard, 3) => HandType::FourOfAKind,
            (HandType::HighCard, 4) => HandType::FiveOfAKind,
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,
            (HandType::OnePair, 2) => HandType::FourOfAKind,
            (HandType::OnePair, 3) => HandType::FiveOfAKind,
            (HandType::TwoPair, 1) => HandType::FullHouse,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind,
            (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards {
            write!(f, "{card}")?;
        }
        write!(f, " {}", self.bid)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::Jack => 'J',
            Card::Tit => 'T',
            Card::Nine => '8',
            Card::Eight => '7',
            Card::Seven => '6',
            Card::Six => '5',
            Card::Five => '4',
            Card::Four => '3',
            Card::Three => '2',
            Card::Two => '1',
        };
        f.write_char(char)
    }
}

fn hand_type_for_cards(cards: &[Card]) -> HandType {
    let counts = card_counts(cards);

    match counts[0].1 {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 if counts.len() > 1 && counts[1].1 == 2 => HandType::FullHouse,
        3 => HandType::ThreeOfAKind,
        2 if counts.len() > 1 && counts[1].1 == 2 => HandType::TwoPair,
        2 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn card_counts(cards: &[Card]) -> Vec<(Card, u8)> {
    let mut counts = cards
        .iter()
        .fold(HashMap::new(), |mut acc, v| {
            *acc.entry(*v).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .collect::<Vec<_>>();
    counts.sort_by_key(|(_, count)| Reverse(*count));
    counts
}

fn joker_cmp(a: &Card, b: &Card) -> Ordering {
    match (*a, *b) {
        (Card::Jack, Card::Jack) => Ordering::Equal,
        (Card::Jack, _) => Ordering::Less,
        (_, Card::Jack) => Ordering::Greater,
        _ => a.cmp(b),
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    (
        separated(1.., (repeat(5..=5, parse_card), " ", dec_int), "\n"),
        multispace0,
    )
        .map(|(lines, _): (Vec<_>, _)| {
            lines
                .into_iter()
                .map(|(cards, _, bid): (Vec<_>, _, _)| Hand {
                    cards: cards.try_into().unwrap(),
                    bid,
                })
                .collect()
        })
        .parse(input)
        .unwrap()
}

fn parse_card(input: &mut &str) -> winnow::Result<Card> {
    dispatch! {any;
    'A' => empty.value(Card::Ace),
    'K' => empty.value(Card::King),
    'Q' => empty.value(Card::Queen),
    'J' => empty.value(Card::Jack),
    'T' => empty.value(Card::Tit),
    '9' => empty.value(Card::Nine),
    '8' => empty.value(Card::Eight),
    '7' => empty.value(Card::Seven),
    '6' => empty.value(Card::Six),
    '5' => empty.value(Card::Five),
    '4' => empty.value(Card::Four),
    '3' => empty.value(Card::Three),
    '2' => empty.value(Card::Two),
    _ => fail,
    }
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "6440");
        assert_eq!(part2.to_string(), "5905");
    }
}
