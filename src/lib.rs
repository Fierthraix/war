#![feature(universal_impl_trait)]

pub mod deck;

use deck::*;
use std::collections::LinkedList;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
        if self.player1.deck.len() < 1 || self.player2.deck.len() < 1 {
            self.game_over = true;
        }
        if self.game_over { return; }
        let cards = (self.player1.draw(), self.player2.draw());
        self.turn_inner(cards, LinkedList::new());
    }
    fn turn_inner(&mut self, cards: (WarCard, WarCard), mut for_grabs: LinkedList<WarCard>) {
        let (card1, card2) = cards;
        if card1 > card2 {
            // Player 1 gets the cards
            self.player1.append(&mut for_grabs);
            self.player1.add(card1);
            self.player1.add(card2);
        } else if card2 > card1 {
            // Player 2 wins the round
            self.player2.append(&mut for_grabs);
            self.player2.add(card1);
            self.player2.add(card2);
        } else {
            // WAAAGGHH!
            if self.player1.deck.len() < 4 || self.player2.deck.len() < 4 {
                // One of the players doesn't have enough cards
                self.game_over = true;
                return;
            }
            // Add the current cards to `for_grabs`
            for_grabs.push_back(card1);
            for_grabs.push_back(card2);
            // Draw three cards from each player
            for _ in 0..3 {
                for_grabs.push_back(self.player1.draw());
                for_grabs.push_back(self.player2.draw());
            }
            // Repeat the turn
            let cards = (self.player1.draw(), self.player2.draw());
            self.turn_inner(cards, for_grabs);
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    fn append(&mut self, cards: &mut LinkedList<WarCard>) {
        self.deck.append(cards);
    }
}

#[test]
fn test_turns_work() {
    fn from_list(cards: &str) -> Player {
        let mut deck = LinkedList::new();
        for card in cards.split(", ") {
            let (val, suit) = card.split_at(card.len() - 1);
            let suit = match suit {
                "s" => CardSuit::Spades,
                "d" => CardSuit::Diamonds,
                "c" => CardSuit::Clubs,
                "h" => CardSuit::Hearts,
                _ => panic!(),
            };
            let val = match val {
                "k" => CardValue::King,
                "q" => CardValue::Queen,
                "j" => CardValue::Jack,
                "a" => CardValue::Ace,
                _ => {
                    if let Ok(n) = val.parse::<u8>() {
                        CardValue::Num(n)
                    } else { panic!() }}
            };
            deck.push_back(WarCard(Card{ val: val, suit: suit }));
        }
        Player{ deck: deck }
    }
    let mut game = WarGame {
        player1: from_list("ks, kh, 2s, js, 6h, 3h, 4s, 8h, 9d, 3c, 5d, kc, 10h, 7d"),
        player2: from_list("9c, as, 7c, qs, 6d, 8d, 3d, 4c, ah, 4h, 5c, qc, 10c, 2d"),
        game_over: false,
    };

    game.turn();
    let result = WarGame {
        player1: from_list("kh, 2s, js, 6h, 3h, 4s, 8h, 9d, 3c, 5d, kc, 10h, 7d, ks, 9c"),
        player2: from_list("as, 7c, qs, 6d, 8d, 3d, 4c, ah, 4h, 5c, qc, 10c, 2d"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("2s, js, 6h, 3h, 4s, 8h, 9d, 3c, 5d, kc, 10h, 7d, ks, 9c"),
        player2: from_list("7c, qs, 6d, 8d, 3d, 4c, ah, 4h, 5c, qc, 10c, 2d, kh, as"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("js, 6h, 3h, 4s, 8h, 9d, 3c, 5d, kc, 10h, 7d, ks, 9c"),
        player2: from_list("qs, 6d, 8d, 3d, 4c, ah, 4h, 5c, qc, 10c, 2d, kh, as, 2s, 7c"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("6h, 3h, 4s, 8h, 9d, 3c, 5d, kc, 10h, 7d, ks, 9c"),
        player2: from_list("6d, 8d, 3d, 4c, ah, 4h, 5c, qc, 10c, 2d, kh, as, 2s, 7c, js, qs"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("3c, 5d, kc, 10h, 7d, ks, 9c, 6h, 6d, 3h, 8d, 4s, 3d, 8h, 4c, 9d, ah"),
        player2: from_list("4h, 5c, qc, 10c, 2d, kh, as, 2s, 7c, js, qs"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("5d, kc, 10h, 7d, ks, 9c, 6h, 6d, 3h, 8d, 4s, 3d, 8h, 4c, 9d, ah"),
        player2: from_list("5c, qc, 10c, 2d, kh, as, 2s, 7c, js, qs, 3c, 4h"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("8d, 4s, 3d, 8h, 4c, 9d, ah"),
        player2: from_list("qs, 3c, 4h, 5d, 5c, kc, qc, 10h, 10c, 7d, 2d, ks, kh, 9c, as, 6h, 2s, 6d, 7c, 3h, js"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("4s, 3d, 8h, 4c, 9d, ah"),
        player2: from_list("3c, 4h, 5d, 5c, kc, qc, 10h, 10c, 7d, 2d, ks, kh, 9c, as, 6h, 2s, 6d, 7c, 3h, js, 8d, qs"),
        game_over: false,
    };
    assert_eq!(game, result);

    game.turn();
    let result = WarGame {
        player1: from_list("3d, 8h, 4c, 9d, ah, 4s, 3c"),
        player2: from_list("4h, 5d, 5c, kc, qc, 10h, 10c, 7d, 2d, ks, kh, 9c, as, 6h, 2s, 6d, 7c, 3h, js, 8d, qs"),
        game_over: false,
    };
    assert_eq!(game, result);
}
