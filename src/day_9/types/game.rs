use crate::day_9::types::*;
use crate::day_9::utils;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Error};

pub type Winner = Player;

impl Game {
    /// Creates a new game from given rules
    pub fn new(rules: GameRules) -> Game {
        Game {
            rules,
            players: (0..rules.num_players).map(|i| Player{id: i, score: 0}).collect(),
            current_player_id: 0,
            circle: Circle{marbles: (0..1).collect(), current_marble_index: 0, next_marble: 1},
            round: 0,
        }
    }

    pub fn play(mut self) -> Winner {
        while !self.is_finished() {
            self.play_one_round();
        }

        let winner_id = self.players.iter().fold(&self.players[0], |winner, player| {
            if winner.score >= player.score {
                winner
            } else {
                player
            }
        }).id;

        self.players.remove(winner_id)
    }

    fn play_one_round(&mut self) {
        let score = self.circle.play_next_marble();

        self.players[self.current_player_id].score += score;
        self.current_player_id = utils::circular_index(self.current_player_id as isize + 1, &self.players);
    }

    fn is_finished(&self) -> bool {
        self.circle.current_marble_index > self.rules.num_marbles
    }
}


impl Debug for Game {
    /// Custom string visualization
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let curr_player = &self.players[self.current_player_id];
        let mut text = format!("Player [{}]  -  Score [{}]\n", curr_player.id, curr_player.score);

        let circle = &self.circle;
        for i in 0..circle.marbles.len() {
            if i == circle.current_marble_index {
                text += format!("({}) | ", &circle.marbles[i]).as_str();
            } else {
                text += format!("{} | ", &circle.marbles[i]).as_str();
            }
        }

        f.write_str(text.as_str())
    }
}
