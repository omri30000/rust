const ROUNDS_NUMBER: usize = 10;
const PINS_AMOUNT: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
enum Throw {
    First,
    Second,
    FillBall
}

pub struct BowlingGame {
    current_round: usize,
    current_throw: Throw,
    frames: [u16; ROUNDS_NUMBER],
    is_fill_ball: bool,
    is_game_complete: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_round: 0,
            current_throw: Throw::First,
            frames: [0; ROUNDS_NUMBER],
            is_fill_ball: false,
            is_game_complete: false
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete {
            return Err(Error::GameComplete);
        }
        if self.frames[self.current_round] + pins > PINS_AMOUNT {
            return Err(Error::NotEnoughPinsLeft);
        }
        
        self.frames[self.current_round] += pins;
        println!("record that {} pins have been scored, in round {}, {:?} throw", pins, self.current_round, self.current_throw);
        self.set_next_roll();

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete {
            return None;
        }
        let mut sum: u16 = 0;
        for frame in self.frames.iter() {
            sum += frame;
        }

        Some(sum)
    }

    /// Set the Throw type for the next roll.
    /// first -> second.
    /// second -> first or fill ball if this is the last round and there has being a spare or a strike.
    /// fill ball -> fill ball since if exists, is the last roll for sure.
    fn set_next_roll(&mut self) {
        match self.current_throw {
            Throw::First => {
                if self.frames[self.current_round] == PINS_AMOUNT
                {
                    // There has been a strike.
                    self.set_next_round();
                } else {
                    self.current_throw = Throw::Second;
                }
            }
            Throw::Second => {
                if self.is_fill_ball {
                    self.current_throw = Throw::FillBall;
                } else {
                    self.current_throw = Throw::First;
                    self.set_next_round();
                }
            }
            Throw::FillBall => self.is_game_complete = true
        }
    }

    fn set_next_round(&mut self) {
        if self.current_round == ROUNDS_NUMBER - 1 {
            self.is_game_complete = true;
        } else {
            self.current_round += 1 
        }
    }
}
