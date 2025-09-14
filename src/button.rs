use core::convert::Infallible;

use embedded_hal::digital::InputPin;

use crate::timer::CustomTimer;

pub struct Button<T, P, A>
where
    T: CustomTimer,
    P: InputPin<Error = Infallible>,
    A: FnMut(),
{
    timer: T,
    pin: P,
    action: A,
}

impl<T, P, A> Button<T, P, A>
where
    T: CustomTimer,
    P: InputPin<Error = Infallible>,
    A: FnMut(),
{
    pub const fn new(timer: T, pin: P, action: A) -> Self {
        Button { timer, pin, action }
    }

    pub fn update(&mut self, elapsed_ms: u32) -> bool {
        let is_pressed = self.is_pressed();
        if (self.timer).update(is_pressed, elapsed_ms) {
            self.trigger();
            true
        } else {
            false
        }
    }

    fn is_pressed(&mut self) -> bool {
        self.pin.is_low().unwrap_or(false)
    }

    fn trigger(&mut self) {
        (self.action)()
    }
}
