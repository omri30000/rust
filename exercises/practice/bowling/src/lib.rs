const ROUNDS_NUMBER: usize = 10;
const THROW_TYPES_AMOUNT: usize = 3;
const PINS_AMOUNT: u16 = 10;

const FIRST_THROW: usize = 0;
const SECOND_THROW: usize = 1;
const FILL_BALL_THROW: usize = 2;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
enum ThrowType {
    First,
    Second,
    FillBall,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RoundResult {
    Regular,
    Spare,
    Strike,
}

#[derive(Clone, Copy)]
struct Frame {
    pub points: [u16; THROW_TYPES_AMOUNT],
}
pub struct BowlingGame {
    current_round: usize,
    current_throw: ThrowType,
    frames: [Frame; ROUNDS_NUMBER],
    extra_points: u16,
    is_fill_ball: bool,
    is_game_complete: bool,
    two_rounds_ago_result: Option<RoundResult>,
    previous_round_result: Option<RoundResult>,
}

impl Frame {
    pub fn sum_points(&self) -> u16 {
        let mut sum = 0;
        for throw in self.points.iter() {
            sum += throw;
        }
        sum
    }

    pub fn is_strike(&self) -> bool {
        self.points[FIRST_THROW] == PINS_AMOUNT
    }

    pub fn is_spare(&self) -> bool {
        self.points[FIRST_THROW] + self.points[SECOND_THROW] == PINS_AMOUNT
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_round: 0,
            current_throw: ThrowType::First,
            frames: [Frame {
                points: [0; THROW_TYPES_AMOUNT],
            }; ROUNDS_NUMBER],
            extra_points: 0,
            is_fill_ball: false,
            is_game_complete: false,
            two_rounds_ago_result: None,
            previous_round_result: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete {
            return Err(Error::GameComplete);
        }
        if !self.is_pins_input_valid(pins) {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.add_pins_to_frames(pins);
        self.set_next_roll();

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete {
            return None;
        }
        let mut sum: u16 = 0;
        for frame in self.frames.iter() {
            sum += frame.sum_points();
        }
        sum += self.extra_points;

        Some(sum)
    }

    /// Set the ThrowType type for the next roll.
    /// first -> second.
    /// second -> first or fill ball if this is the last round and there has being a spare or a strike.
    /// fill ball -> fill ball since if exists, is the last roll for sure.
    fn set_next_roll(&mut self) {
        match self.current_throw {
            ThrowType::First => {
                if self.frames[self.current_round].is_strike() {
                    // There has been a strike.
                    self.set_strike();
                    if self.is_last_round() {
                        self.current_throw = ThrowType::Second;
                    } else {
                        self.move_to_next_round();
                    }
                } else {
                    self.current_throw = ThrowType::Second;
                }
            }
            ThrowType::Second => {
                if self.frames[self.current_round].is_spare() {
                    self.set_spare();
                } else {
                    self.two_rounds_ago_result = self.previous_round_result;
                    self.previous_round_result = Some(RoundResult::Regular);
                }
                if self.is_fill_ball {
                    self.current_throw = ThrowType::FillBall;
                } else {
                    self.current_throw = ThrowType::First;
                    self.move_to_next_round();
                }
            }
            ThrowType::FillBall => self.is_game_complete = true,
        }
    }

    fn move_to_next_round(&mut self) {
        if self.current_round == ROUNDS_NUMBER - 1 {
            self.is_game_complete = true;
        } else {
            self.current_round += 1
        }
    }

    fn add_pins_to_frames(&mut self, pins: u16) {
        match self.current_throw {
            ThrowType::First => {
                self.frames[self.current_round].points[FIRST_THROW] += pins;
                match self.previous_round_result {
                    Some(result) => match result {
                        RoundResult::Spare => {
                            self.extra_points += pins;
                        }
                        RoundResult::Strike => {
                            self.extra_points += pins;
                            if let Some(round_result) = self.two_rounds_ago_result {
                                if round_result == RoundResult::Strike {
                                    self.extra_points += pins;
                                }
                            }
                        }
                        _ => (),
                    },
                    None => (),
                }
            }
            ThrowType::Second => {
                self.frames[self.current_round].points[SECOND_THROW] += pins;
                if let Some(result) = self.previous_round_result {
                    if result == RoundResult::Strike {
                        self.extra_points += pins;
                    }
                }
            }
            ThrowType::FillBall => self.frames[self.current_round].points[FILL_BALL_THROW] += pins,
        }
    }

    fn is_pins_input_valid(&self, pins: u16) -> bool {
        match self.current_throw {
            ThrowType::First => pins <= PINS_AMOUNT,
            ThrowType::Second => {
                self.frames[self.current_round].points[FIRST_THROW] + pins <= PINS_AMOUNT
                    || (self.frames[self.current_round].is_strike() && pins <= PINS_AMOUNT)
            }
            ThrowType::FillBall => {
                (self.frames[self.current_round].is_spare() && pins <= PINS_AMOUNT)
                    || (self.frames[self.current_round].is_strike()
                        && self.frames[self.current_round].points[SECOND_THROW] + pins
                            <= PINS_AMOUNT)
                    || (self.frames[self.current_round].is_strike()
                        && self.frames[self.current_round].points[SECOND_THROW] == PINS_AMOUNT
                        && pins <= PINS_AMOUNT)
            }
        }
    }

    fn is_last_round(&self) -> bool {
        self.current_round == ROUNDS_NUMBER - 1
    }

    fn set_irregular_result(&mut self, result: RoundResult) {
        if self.is_last_round() {
            self.is_fill_ball = true;
        } else {
            self.two_rounds_ago_result = self.previous_round_result;
            self.previous_round_result = Some(result);
        }
    }

    fn set_strike(&mut self) {
        self.set_irregular_result(RoundResult::Strike);
    }

    fn set_spare(&mut self) {
        self.set_irregular_result(RoundResult::Spare);
    }
}
