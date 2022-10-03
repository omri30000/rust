pub const THROW_TYPES_AMOUNT: usize = 3;
pub const PINS_AMOUNT: u16 = 10;

pub const FIRST_THROW: usize = 0;
pub const SECOND_THROW: usize = 1;

#[derive(Clone, Copy)]
pub struct Frame {
    pub points: [u16; THROW_TYPES_AMOUNT],
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
