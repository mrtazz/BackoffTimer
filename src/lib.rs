use std::time::Duration;
use std::thread;

/// BackoffTimer implements a timer that waits a certain amount and then backs off
/// until the configured max_wait_time (specified in seconds) is reached or passed.
///
/// Different specific timers implement their own version of back algorithms
pub trait BackoffTimer {
    fn wait(&mut self) -> u64;
    fn is_done(&self) -> bool;
}

/// ```
/// use backoff_timer::BackoffTimer;
/// use backoff_timer::exponential_backoff_timer;
///
/// let mut timer = exponential_backoff_timer(5);
/// let mut waited = timer.wait();
/// assert_eq!(waited, 2);
/// assert_eq!(timer.is_done(), false);
/// waited = timer.wait();
/// assert_eq!(waited, 4);
/// assert_eq!(timer.is_done(), true);
/// ```
pub struct ExponentialBackoffTimer {
    waited_time:   u64,
    wait_time:     u64,
    max_wait_time: u64,
}

/// new returns a new ExponentialBackoffTimer configured with the passed in max_wait_time in seconds
pub fn exponential_backoff_timer(max_wait_time: u64) -> ExponentialBackoffTimer {
    ExponentialBackoffTimer {
        max_wait_time: max_wait_time,
        wait_time: 2,
        waited_time: 0,
    }
}


impl BackoffTimer for ExponentialBackoffTimer {

    /// wait is a blocking function that waits for the appropriate amount of time depending on the
    /// timers internal state. It returns the time waited which is 0 if the timer has reached its
    /// max_wait_time
    fn wait(&mut self) -> u64 {
        if self.is_done() {
            return 0;
        }
        thread::sleep(Duration::from_secs(self.wait_time));
        let time_waited = self.wait_time;
        self.wait_time = self.wait_time.pow(2);
        self.waited_time = self.waited_time + self.wait_time;

        return time_waited;
    }

    /// is_done is a helper method to determine whether the timer is done or still has turns to
    /// wait.
    fn is_done(&self) -> bool {
        self.waited_time >= self.max_wait_time
    }

}



#[cfg(test)]
mod tests {
    use super::BackoffTimer;
    use super::exponential_backoff_timer;

    #[test]
    fn wait_for_5_seconds() {
        let mut timer = exponential_backoff_timer(5);
        let mut waited = timer.wait();
        assert_eq!(waited, 2);
        assert_eq!(timer.is_done(), false);
        waited = timer.wait();
        assert_eq!(waited, 4);
        assert_eq!(timer.is_done(), true);
    }
}
