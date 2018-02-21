#![feature(universal_impl_trait)]

pub mod deck;

use deck::*;
use std::collections::LinkedList;

#[derive(Clone, PartialEq)]
pub struct WarCard(Card);

impl From<Card> for WarCard {
    fn from(card: Card) -> Self {
        WarCard(card)
    }
}

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

pub struct WarGame {
    player1: Player,
    player2: Player,
    pub game_over: bool,
}

impl WarGame {
    pub fn new() -> Self {
        let mut deck = Deck::default();
        deck.shuffle();

        let (deck1, deck2) = deck.split();

        WarGame {
            player1: Player::new(&deck1),
            player2: Player::new(&deck2),
            game_over: false,
        }
    }
    pub fn turn(&mut self) {
        if self.game_over { return; }
        // Decks aren't empty, we check for this at the end of every turn and set `game_over`
        let cards = (self.player1.draw(), self.player2.draw());
        self.turn_inner(cards, vec![]);
    }
    fn turn_inner(&mut self, cards: (WarCard, WarCard), mut for_grabs: Vec<WarCard>) {
        let (card1, card2) = cards;
        if card1 > card2 {
            // Player 1 gets the cards
            self.player1.add(card1);
            self.player1.add(card2);
            self.player1.append(for_grabs)
        } else if card2 > card1 {
            // Player 2 wins the round
            self.player2.add(card1);
            self.player2.add(card2);
        } else {
            // WAAAGGHH!
            if self.player1.deck.len() < 4 || self.player2.deck.len() < 4 {
                // One of the players doesn't have enough cards
                self.game_over = true;
                return;
            }
            // Draw three cards from each player
            for _ in 0..3 {
                for_grabs.push(self.player1.draw());
            }
            for _ in 0..3 {
                for_grabs.push(self.player2.draw());
            }
            // Repeat the turn
            let cards = (self.player1.draw(), self.player2.draw());
            self.turn_inner(cards, for_grabs);
        }
    }
}

struct Player {
    deck: LinkedList<WarCard>,
}

impl Player {
    fn new<T>(deck: &[T]) -> Self
        where T: Into<WarCard> + Clone
        {
            Player {
                deck: deck.iter().map(|card| (*card).clone().into()).collect::<LinkedList<_>>()
            }
        }
    fn draw(&mut self) -> WarCard {
        self.deck.pop_front().unwrap()
    }
    fn add(&mut self, card: WarCard) {
        self.deck.push_back(card)
    }
    fn append(&mut self, cards: impl IntoIterator<Item = WarCard>) {
        cards.into_iter().for_each(|card| self.deck.push_back(card));
    }
}
