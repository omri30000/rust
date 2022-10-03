/// The amount of different throw types available (first, second, fill ball)
pub const THROW_TYPES: usize = 3;
/// The maximum amount of pins which can be scored in a single throw
pub const PINS_AMOUNT: u16 = 10;

/// The index of the first ball throw in a points array of Frame structure
pub const FIRST_THROW: usize = 0;
/// The index of the second ball throw in a points array of Frame structure
pub const SECOND_THROW: usize = 1;

#[derive(Debug, PartialEq, Eq)]
/// The type of the throw that the system supports
pub enum ThrowType {
    First,
    Second,
    FillBall,
}

#[derive(Clone, Copy)]
/// Represents a single turn, a frame consists of 3 ball throws, while the second
/// and third might be skipped according to the rules of the game (strike, spare, fill ball)
pub struct Frame {
    /// The score that the player scored in each throw
    pub points: [u16; THROW_TYPES],
}
impl Frame {
    /// Summarize the score for the whole turn (max of 3 throws)
    pub fn sum_points(&self) -> u16 {
        self.points.iter().sum()
    }

    /// Check whether there has been a strike in the current turn
    pub fn is_strike(&self) -> bool {
        self.points[FIRST_THROW] == PINS_AMOUNT
    }

    /// Check whether there has been a spare in the current turn
    pub fn is_spare(&self) -> bool {
        self.points[FIRST_THROW] + self.points[SECOND_THROW] == PINS_AMOUNT
    }
}
