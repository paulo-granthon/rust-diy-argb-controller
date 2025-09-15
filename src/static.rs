use avr_device::interrupt;
use core::cell::UnsafeCell;

pub struct Static<T>(UnsafeCell<T>);
unsafe impl<T> Sync for Static<T> {}

impl<T: 'static> Static<T> {
    /// Run `f` with a mutable reference to the inner State inside a critical section.
    /// This blocks interrupts while `f` runs (prevents interrupt handlers concurrently accessing the state).
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        interrupt::free(|_| {
            // Safe: we have exclusive access because interrupts are disabled in this closure.
            let ptr = self.0.get();
            let s: &mut T = unsafe { &mut *ptr };
            f(s)
        })
    }

    pub const fn new(inner: T) -> Self {
        Static(UnsafeCell::new(inner))
    }
}
