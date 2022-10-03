pub const THROW_TYPES_AMOUNT: usize = 3;
pub const PINS_AMOUNT: u16 = 10;

pub const FIRST_THROW: usize = 0;
pub const SECOND_THROW: usize = 1;


#[derive(Clone, Copy)]
/// Represents a single turn, a frame consists of 3 ball throws, while the second
/// and third might be skipped according to the rules of the game (strike, spare, fill ball)
pub struct Frame {
    /// The score that the player scored in each throw
    pub points: [u16; THROW_TYPES_AMOUNT],
}
impl Frame {
    /// Summarize the score for the whole turn (max of 3 throws)
    pub fn sum_points(&self) -> u16 {
        let mut sum = 0;
        for throw in self.points.iter() {
            sum += throw;
        }
        sum
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
