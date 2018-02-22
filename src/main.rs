extern crate rayon;
extern crate war;

use war::WarGame;
use rayon::prelude::*;

use std::collections::HashSet;

const NUM_ROUNDS: usize = 100_000;
const MAX_TURNS: usize = 100_000;

#[derive(Clone)]
enum EndGame {
    Finished(usize),
    Repeated(usize),
    TimedOut,
}

fn main() {
    let totals = (0..NUM_ROUNDS).into_par_iter().map(|_|{
        let mut game = WarGame::new();
        let mut turns = 0usize;

        let mut seen_states = HashSet::new();

        while !game.game_over {
            if turns >= MAX_TURNS {
                return EndGame::TimedOut;
            }
            game.turn();
            if seen_states.contains(&game) {
                return EndGame::Repeated(turns)
            } else {
                seen_states.insert(game.clone());
            }
            turns += 1;
        }
        EndGame::Finished(turns)
    }).collect::<Vec<EndGame>>();

    // Get the average number of turns for finished games
    let average_turns = totals.iter().filter_map(|state| {
        if let EndGame::Finished(turns) = *state {
            Some(turns)
        } else {
            None
        }
    }).collect::<Vec<usize>>();
    println!("Average num of turns for games that finished: {}",
             average_turns.iter().map(|&n| n).sum::<usize>() / average_turns.len());

    // Get how many games repeated
    let num_repeats = totals.iter().filter_map(|state| {
        if let EndGame::Repeated(turns) = *state {
            Some(turns)
        } else {
            None
        }
    }).collect::<Vec<usize>>();
    println!("{}/ {} ({}%) repeated after an average of {} turns",
    num_repeats.len(), MAX_TURNS,
    num_repeats.len() as f64 / MAX_TURNS as f64 * 100.,
    num_repeats.iter().map(|&n| n).sum::<usize>() / num_repeats.len());


    // Get how many didn't end yet
    let num_unfinished = totals.iter().map(|state| {
        if let EndGame::TimedOut = *state {
            1
        } else {
            0
        }
    }).sum::<usize>();
    println!("{} / {} ({}%) of games didn't finish after {} rounds",
    num_unfinished, MAX_TURNS,
    num_unfinished as f64 / MAX_TURNS as f64 * 100.,
    MAX_TURNS);
}
