#[derive(PartialEq, Eq, Debug)]
pub struct WaitError;

/// BackoffTimer implements a timer that waits a certain amount and then backs off
/// until the configured max_wait_time (specified in seconds) is reached or passed.
///
/// Different specific timers implement their own version of back algorithms
pub trait BackoffTimer {
    fn wait(&mut self) -> Result<u64, WaitError>;
    fn is_done(&self) -> bool;
    fn new(u64) -> Self;
}

pub mod exponential;
pub use exponential::ExponentialBackoffTimer;
