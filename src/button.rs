use crate::timer::CustomTimer;

pub struct Button<T, P, A>
where
    T: CustomTimer,
    P: Fn() -> bool,
    A: FnMut(),
{
    timer: T,
    press: P,
    action: A,
}

impl<T, P, A> Button<T, P, A>
where
    T: CustomTimer,
    P: Fn() -> bool,
    A: FnMut(),
{
    pub const fn new(timer: T, press: P, action: A) -> Self {
        Button {
            timer,
            press,
            action,
        }
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

    fn is_pressed(&self) -> bool {
        (self.press)()
    }

    fn trigger(&mut self) {
        (self.action)()
    }
}
