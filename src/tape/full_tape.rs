use super::half_tape::HalfTape;
use std::usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
pub struct Tape {
    which: Direction,
    left: HalfTape,
    right: HalfTape,
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            which: Direction::RIGHT,
            left: HalfTape::new(),
            right: HalfTape::new(),
        }
    }

    pub fn move_pointer_by(&mut self, by: i128) {
        let move_by: i128 = if self.which == Direction::LEFT {
            by.checked_neg().unwrap()
        } else {
            by
        };

        // Simple cases: moving entirely within one tape
        if self.which == Direction::RIGHT
            && self.on_current_tape(|tape| tape.get_pointer()) as i128 + move_by >= 0
        {
            // This means that we're moving on the right tape in such a way that we stay on the right tape.
            return self.on_current_tape(|tape| tape.move_pointer_by(move_by));
        } else if self.which == Direction::LEFT
            && self.on_current_tape(|tape| tape.get_pointer()) as i128 + move_by > 0
        {
            // This means that we're moving on the left tape in such a way that we stay on the left tape.
            return self.on_current_tape(|tape| tape.move_pointer_by(move_by));
        }

        // Okay, we know we need to swap between tapes now, so that's cool
        let new_position =
            (move_by - self.on_current_tape(|tape| tape.get_pointer()) as i128).abs();

        if new_position > usize::MAX as i128 {
            panic!(
                "Out of memory! Tried to allocate more than {} bytes to a half tape.",
                usize::MAX
            )
        }

        self.on_current_tape(|tape| tape.move_pointer_to(0));
        self.which = match self.which {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        };
        self.on_current_tape(|tape| tape.move_pointer_to(new_position as usize));
    }

    pub fn get_value(&mut self) -> u8 {
        self.on_current_tape(|tape| tape.get_value())
    }

    pub fn set_value(&mut self, value: u8) {
        self.on_current_tape(|tape| tape.set_value(value))
    }

    pub fn increment_value(&mut self, by: u8) {
        self.on_current_tape(|tape| tape.increment_value(by))
    }

    pub fn decrement_value(&mut self, by: u8) {
        self.on_current_tape(|tape| tape.decrement_value(by))
    }

    fn on_current_tape<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut HalfTape) -> T,
    {
        match self.which {
            Direction::LEFT => f(&mut self.left),
            Direction::RIGHT => f(&mut self.right),
        }
    }
}
