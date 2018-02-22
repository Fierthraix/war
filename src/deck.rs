extern crate rand;

use self::rand::{thread_rng, Rng};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Card {
    pub suit: CardSuit,
    pub val: CardValue,
}

use std::fmt;
impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = match self.suit {
            CardSuit::Spades => "Spades",
            CardSuit::Hearts => "Hearts",
            CardSuit::Clubs => "Clubs",
            CardSuit::Diamonds => "Diamonds",
        };
        let val = match self.val {
            CardValue::King => "King".to_string(),
            CardValue::Queen => "Queen".to_string(),
            CardValue::Jack => "Jack".to_string(),
            CardValue::Ace => "Ace".to_string(),
            CardValue::Num(n) => {
                format!("{}", n)
            },
        };
        write!(f, "{} of {}", val, suit)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CardSuit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CardValue {
    King,
    Queen,
    Jack,
    Num(u8),
    Ace
}

pub struct Deck([Card; 52]);

impl Deck {
    pub fn shuffle(&mut self) {
        thread_rng().shuffle(&mut self.0);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn split(&self) -> (&[Card], &[Card]) {
        (&self.0[..self.len()/2], &self.0[self.len()/2..])
    }
    pub fn into_inner(self) -> [Card; 52] {
        self.0
    }
}

impl ::std::ops::Index<usize> for Deck {
    type Output = Card;
    fn index(&self, val: usize) -> &Card {
        &self.0[val]
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck([
             Card{suit: CardSuit::Spades, val: CardValue::King},
             Card{suit: CardSuit::Spades, val: CardValue::Queen},
             Card{suit: CardSuit::Spades, val: CardValue::Jack},
             Card{suit: CardSuit::Spades, val: CardValue::Num(10)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(9)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(8)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(7)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(6)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(5)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(4)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(3)},
             Card{suit: CardSuit::Spades, val: CardValue::Num(2)},
             Card{suit: CardSuit::Spades, val: CardValue::Ace},

             Card{suit: CardSuit::Diamonds, val: CardValue::King},
             Card{suit: CardSuit::Diamonds, val: CardValue::Queen},
             Card{suit: CardSuit::Diamonds, val: CardValue::Jack},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(10)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(9)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(8)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(7)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(6)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(5)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(4)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(3)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Num(2)},
             Card{suit: CardSuit::Diamonds, val: CardValue::Ace},

             Card{suit: CardSuit::Clubs, val: CardValue::King},
             Card{suit: CardSuit::Clubs, val: CardValue::Queen},
             Card{suit: CardSuit::Clubs, val: CardValue::Jack},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(10)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(9)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(8)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(7)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(6)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(5)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(4)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(3)},
             Card{suit: CardSuit::Clubs, val: CardValue::Num(2)},
             Card{suit: CardSuit::Clubs, val: CardValue::Ace},

             Card{suit: CardSuit::Hearts, val: CardValue::King},
             Card{suit: CardSuit::Hearts, val: CardValue::Queen},
             Card{suit: CardSuit::Hearts, val: CardValue::Jack},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(10)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(9)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(8)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(7)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(6)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(5)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(4)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(3)},
             Card{suit: CardSuit::Hearts, val: CardValue::Num(2)},
             Card{suit: CardSuit::Hearts, val: CardValue::Ace},
             ])
    }
}
