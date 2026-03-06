use std::borrow::Cow;
use std::time::{Duration, Instant};

/// A simple timer that prints time taken on drop
pub struct TimerPrintOnDrop<'a> {
    name: Cow<'a, str>,
    start: Instant,
    disabled: bool,
}

impl<'a> TimerPrintOnDrop<'a> {
    #[inline]
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            disabled: false,
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for TimerPrintOnDrop<'_> {
    fn drop(&mut self) {
        if !self.disabled {
            println!("{} took {:.2}s", self.name, self.elapsed().as_secs_f64())
        }
    }
}
