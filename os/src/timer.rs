//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use core::sync::atomic::{AtomicUsize, Ordering};
use riscv::register::time;
/// The number of ticks per second
const TICKS_PER_SEC: usize = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: usize = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: usize = 1_000_000;
/// The number of milliseconds per timer tick.
const MSEC_PER_TICK: usize = MSEC_PER_SEC / TICKS_PER_SEC;

static TIME_MS: AtomicUsize = AtomicUsize::new(0);

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    TIME_MS.load(Ordering::Relaxed)
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    get_time_ms() * (MICRO_PER_SEC / MSEC_PER_SEC)
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/// Advance the software clock by the given number of milliseconds.
pub fn advance_time(ms: usize) {
    TIME_MS.fetch_add(ms, Ordering::Relaxed);
}

/// Record one timer tick on the software clock.
pub fn tick() {
    advance_time(MSEC_PER_TICK);
}
