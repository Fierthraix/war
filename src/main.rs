extern crate rayon;
extern crate war;

use war::WarGame;
use rayon::prelude::*;

const NUM_ROUNDS: usize = 1000;
const MAX_TURNS: usize = 10_000_000;

fn main() {
    let totals = (0..NUM_ROUNDS).into_par_iter().filter_map(|_|{
        let mut game = WarGame::new();
        let mut turns = 0usize;

        while !game.game_over {
            if turns >= MAX_TURNS {
                return None;
            }
            game.turn();
            turns += 1;
        }
        Some(turns)
    }).collect::<Vec<_>>();
    let average_turns = totals.iter().map(|&n| n).sum::<usize>() / totals.len();
    println!("Average num of turns: {}", average_turns);
}
