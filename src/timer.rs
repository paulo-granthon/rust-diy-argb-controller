// TODO: allow different numeric sizes for timer
pub trait CustomTimer {
    fn update(&mut self, pressed: bool, elapsed_ms: u32) -> bool;
}

pub struct PressTimer {
    interval_ms: u32,
    acc_ms: u32,
    was_pressed: bool,
}

impl PressTimer {
    pub const fn new(interval_ms: u32) -> Self {
        PressTimer {
            interval_ms,
            acc_ms: 0,
            was_pressed: false,
        }
    }
}

impl CustomTimer for PressTimer {
    fn update(&mut self, pressed: bool, elapsed_ms: u32) -> bool {
        if !pressed {
            // Released: reset state so next press is immediate
            self.was_pressed = false;
            self.acc_ms = 0;
            return false;
        }

        if !self.was_pressed {
            // Edge: released -> pressed => immediate trigger
            self.was_pressed = true;
            self.acc_ms = 0;
            return true;
        }

        // Still held: accumulate time and trigger every interval_ms
        self.acc_ms = self.acc_ms.wrapping_add(elapsed_ms);
        if self.acc_ms >= self.interval_ms {
            // keep remainder for phase correctness
            self.acc_ms = self.acc_ms.wrapping_sub(self.interval_ms);
            return true;
        }

        false
    }
}

pub struct StrictPressTimer {
    interval_ms: u32,
    acc_ms: u32,
    was_pressed: bool,
}

impl StrictPressTimer {
    pub const fn new(interval_ms: u32) -> Self {
        StrictPressTimer {
            interval_ms,
            acc_ms: 0,
            was_pressed: false,
        }
    }
}

impl CustomTimer for StrictPressTimer {
    fn update(&mut self, pressed: bool, elapsed_ms: u32) -> bool {
        if !pressed {
            // Released -> reset
            self.was_pressed = false;
            self.acc_ms = 0;
            return false;
        }

        if !self.was_pressed {
            // Edge: immediate trigger
            self.was_pressed = true;
            self.acc_ms = 0;
            return true;
        }

        // Held: accumulate and fire each interval
        self.acc_ms = self.acc_ms.wrapping_add(elapsed_ms);
        if self.acc_ms >= self.interval_ms {
            self.acc_ms = self.acc_ms.wrapping_sub(self.interval_ms);
            return true;
        }

        false
    }
}
