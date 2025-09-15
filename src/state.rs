use core::cell::Cell;

use crate::r#static::Static;

pub struct State {
    pub brightness: Cell<u8>,
    pub phase_offset: Cell<u8>,
}

impl State {
    pub const fn new() -> Self {
        State {
            brightness: Cell::new(32),
            phase_offset: Cell::new(0),
        }
    }
}

pub static STATE: Static<State> = Static::new(State::new());
