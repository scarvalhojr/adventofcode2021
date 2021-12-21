use std::cmp::max;
use Player::*;

#[derive(Clone, Eq, PartialEq)]
enum Player {
    Player1,
    Player2,
}

struct DeterministicGame {
    position1: u64,
    position2: u64,
    score1: u64,
    score2: u64,
    turn: Player,
}

impl DeterministicGame {
    fn new(player1: u64, player2: u64) -> Self {
        Self {
            position1: player1 - 1,
            position2: player2 - 1,
            score1: 0,
            score2: 0,
            turn: Player1,
        }
    }

    fn play(&mut self, die_sum: u64) {
        if self.turn == Player1 {
            self.position1 = (self.position1 + die_sum) % 10;
            self.score1 += self.position1 + 1;
            self.turn = Player2;
        } else {
            self.position2 = (self.position2 + die_sum) % 10;
            self.score2 += self.position2 + 1;
            self.turn = Player1;
        }
    }

    fn looser_score(&self) -> Option<u64> {
        if self.score1 >= 1_000 {
            Some(self.score2)
        } else if self.score2 >= 1_000 {
            Some(self.score1)
        } else {
            None
        }
    }
}

pub fn part1(player1: u64, player2: u64) -> u64 {
    let mut game = DeterministicGame::new(player1, player2);

    for (die_sum, rolls) in (6..).step_by(9).zip((3..).step_by(3)) {
        game.play(die_sum);
        if let Some(looser_score) = game.looser_score() {
            return rolls * looser_score;
        }
    }

    unreachable!()
}

#[derive(Clone)]
struct QuantumGame {
    position1: u64,
    position2: u64,
    score1: u64,
    score2: u64,
    turn: Player,
    count: u64,
}

impl QuantumGame {
    fn new(player1: u64, player2: u64) -> Self {
        Self {
            position1: player1 - 1,
            position2: player2 - 1,
            score1: 0,
            score2: 0,
            turn: Player1,
            count: 1,
        }
    }

    fn play(&self, die_sum: u64, count: u64) -> Self {
        let mut new_state = self.clone();
        if self.turn == Player1 {
            new_state.position1 = (new_state.position1 + die_sum) % 10;
            new_state.score1 += new_state.position1 + 1;
            new_state.turn = Player2;
        } else {
            new_state.position2 = (new_state.position2 + die_sum) % 10;
            new_state.score2 += new_state.position2 + 1;
            new_state.turn = Player1;
        }
        new_state.count *= count;
        new_state
    }

    fn winner(&self) -> Option<Player> {
        if self.score1 >= 21 {
            Some(Player1)
        } else if self.score2 >= 21 {
            Some(Player2)
        } else {
            None
        }
    }
}

pub fn part2(player1: u64, player2: u64) -> u64 {
    let mut wins1 = 0;
    let mut wins2 = 0;
    let mut stack = vec![QuantumGame::new(player1, player2)];

    // Each player rolls the 3-face die three times. The sum of the three rolls
    // can be between 3 and 9. There is 1 way to get a sum of 3: (1, 1, 1).
    // Similarly, there are 3 ways to get a sum of 4: (1, 1, 2), (1, 2, 1), and
    // (2, 1, 1).

    while let Some(game) = stack.pop() {
        for (die_sum, count) in
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        {
            let new_game = game.play(die_sum, count);
            match new_game.winner() {
                Some(winner) if winner == Player1 => wins1 += new_game.count,
                Some(_winner) => wins2 += new_game.count,
                _ => stack.push(new_game),
            }
        }
    }

    max(wins1, wins2)
}
