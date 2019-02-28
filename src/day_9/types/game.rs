use crate::day_9::types::*;
use crate::day_9::utils;
use std::collections::VecDeque;

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
            finished: false
        }
    }

    pub fn play(&mut self) -> &Winner {
        while !self.finished {
            self.play_one_round();
        }

        let winner = self.players.iter()
            .fold(&self.players[0], |winner, player| {
                if winner.score >= player.score {
                    winner
                } else {
                    player
                }
            });

        winner
    }

    /// Advances the game by one round
    pub fn play_one_round(&mut self) {
        let score = self.circle.play_next_marble();

        self.players[self.current_player_id].score += score;
        self.current_player_id = utils::circular_index(self.current_player_id as isize + 1, &self.players);

        if score == self.rules.last_marble_score {
            self.finished = true
        } else if score > self.rules.last_marble_score * 10 {
            panic!("Last score too high: {} -> {}", score, self.rules.last_marble_score);
        }
    }
}
