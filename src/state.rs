use avr_device::interrupt;
use core::cell::{Cell, UnsafeCell};

pub static STATE: StaticState = StaticState(UnsafeCell::new(State::new()));

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

pub struct StaticState(UnsafeCell<State>);
unsafe impl Sync for StaticState {}

impl StaticState {
    /// Run `f` with a mutable reference to the inner State inside a critical section.
    /// This blocks interrupts while `f` runs (prevents interrupt handlers concurrently accessing the state).
    pub fn with<R>(&self, f: impl FnOnce(&mut State) -> R) -> R {
        interrupt::free(|_| {
            // Safe: we have exclusive access because interrupts are disabled in this closure.
            let ptr = self.0.get();
            let s: &mut State = unsafe { &mut *ptr };
            f(s)
        })
    }
}
