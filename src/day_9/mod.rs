use crate::day_9::utils::play_marble_game;

mod data;
mod utils;

pub fn result_part_one() -> u64 {
    let (num_players, num_marbles) = utils::get_game_rules(data::GAME_RULES);
    play_marble_game(num_players, num_marbles)
}

pub fn result_part_two() -> u64 {
    let (num_players, mut num_marbles) = utils::get_game_rules(data::GAME_RULES);
    num_marbles *= 100;
    play_marble_game(num_players, num_marbles)
}
