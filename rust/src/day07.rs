use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
    Joker = 1,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            'X' => Card::Joker,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandType {
    fn from_cards(cards: &[Card]) -> HandType {
        let mut counts = HashMap::new();
        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        // Special handling for jokers
        if let Some(&joker_count) = counts.get(&Card::Joker) {
            counts.remove(&Card::Joker);
            if joker_count == 5 {
                return HandType::FiveOfAKind;
            }

            if let Some((most_common_card, _)) = counts.iter().max_by_key(|&(_, count)| count) {
                *counts.entry(*most_common_card).or_insert(0) += joker_count;
            }
        }

        match *counts.values().max().unwrap_or(&0) {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if counts.values().any(|&v| v == 2) => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if counts.values().filter(|&&v| v == 2).count() == 2 => HandType::TwoPair,
            2 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    value: (usize, Vec<u8>),
    pub bid: usize,
}

impl Hand {
    pub fn from_string(s: String) -> Option<Hand> {
        let mut parts = s.split_whitespace();
        let card_symbols = parts.next()?;
        let bid = parts.next()?.parse().ok()?;

        let cards: Vec<Card> = card_symbols.chars().map(|c| Card::from_char(c)).collect();

        if cards.len() != 5 {
            println!("Invalid hand: {:?}", cards);
            return None;
        }

        let hand_type = HandType::from_cards(&cards);
        let value = (hand_type as usize, cards.iter().map(|&c| c as u8).collect());

        Some(Hand {
            cards,
            hand_type,
            value,
            bid,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
