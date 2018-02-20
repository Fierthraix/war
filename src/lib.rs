#[derive(PartialEq)]
pub struct Card {
    suit: CardSuit,
    val: CardValue,
}

#[derive(PartialEq)]
pub enum CardSuit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

#[derive(PartialEq)]
pub enum CardValue {
    King,
    Queen,
    Jack,
    Num(u8),
    Ace
}

#[derive(PartialEq)]
pub struct WarCard(Card);

use std::cmp::Ordering::{self, Equal, Greater, Less};

// TODO: add #[cfg()] for aces high
impl PartialOrd<WarCard> for WarCard {
    fn partial_cmp(&self, other: &WarCard) -> Option<Ordering> {
        match self.0.val {
            CardValue::King => match other.0.val {
                CardValue::King => Some(Equal),
                CardValue::Ace => Some(Less),
                _ => Some(Greater)
            },
            CardValue::Queen => match other.0.val {
                CardValue::King => Some(Less),
                CardValue::Queen => Some(Equal),
                _ => Some(Greater),
            },
            CardValue::Jack => match other.0.val {
                CardValue::King | CardValue::Queen => Some(Less),
                CardValue::Jack => Some(Equal),
                _ => Some(Greater)
            },
            CardValue::Num(n) => match other.0.val {
                CardValue::King | CardValue::Queen | CardValue::Jack => Some(Less),
                CardValue::Num(m) => if n > m {
                    Some(Greater)
                } else if n == m {
                    Some(Equal)
                } else {
                    Some(Less)
                }
                _ => Some(Greater)
            },
            CardValue::Ace => match other.0.val {
                CardValue::King => Some(Greater),
                CardValue::Ace => Some(Equal),
                _ => Some(Less)
            }
        }
    }
}
