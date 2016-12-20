pub use BackoffTimer;
pub use WaitError;
use std::time::Duration;
use std::thread;

/// ```
/// use backoff_timer::*;
///
///     let mut timer = ExponentialBackoffTimer::new(6);
///     match timer.wait() {
///         Ok(time_waited) => assert_eq!(time_waited, 2),
///         Err(e) => panic!(e),
///     }
///     assert_eq!(timer.is_done(), false);
///     match timer.wait() {
///         Ok(time_waited) => assert_eq!(time_waited, 4),
///         Err(e) => panic!(e),
///     }
///     assert_eq!(timer.is_done(), true);
/// ```
pub struct ExponentialBackoffTimer {
    waited_time:   u64,
    wait_time:     u64,
    max_wait_time: u64,
}

impl BackoffTimer for ExponentialBackoffTimer {

    /// new returns a new ExponentialBackoffTimer configured with the passed in max_wait_time in seconds
    fn new(max_wait_time: u64) -> ExponentialBackoffTimer {
        ExponentialBackoffTimer {
            max_wait_time: max_wait_time,
            wait_time: 2,
            waited_time: 0,
        }
    }



    /// wait is a blocking function that waits for the appropriate amount of time depending on the
    /// timers internal state. It returns the time waited which is 0 if the timer has reached its
    /// max_wait_time
    fn wait(&mut self) -> Result<u64, WaitError> {
        if self.is_done() {
            return Err(WaitError);
        }
        thread::sleep(Duration::from_secs(self.wait_time));
        let time_waited = self.wait_time;
        self.waited_time = self.waited_time + self.wait_time;
        self.wait_time = self.wait_time.pow(2);

        return Ok(time_waited);
    }

    /// is_done is a helper method to determine whether the timer is done or still has turns to
    /// wait.
    fn is_done(&self) -> bool {
        (self.waited_time + self.wait_time) > self.max_wait_time
    }

}

#[cfg(test)]
mod tests {
    use super::BackoffTimer;
    use super::ExponentialBackoffTimer;
    use super::WaitError;

    #[test]
    fn wait_for_6_seconds() {
        let mut timer = ExponentialBackoffTimer::new(6);
        match timer.wait() {
            Ok(time_waited) => assert_eq!(time_waited, 2),
            Err(e) => panic!(e),
        }
        assert_eq!(timer.is_done(), false);
        match timer.wait() {
            Ok(time_waited) => assert_eq!(time_waited, 4),
            Err(e) => panic!(e),
        }
        assert_eq!(timer.is_done(), true);
    }

    #[test]
    fn cant_wait_twice_for_five() {
        let mut timer = ExponentialBackoffTimer::new(5);
        match timer.wait() {
            Ok(time_waited) => assert_eq!(time_waited, 2),
            Err(e) => panic!(e),
        }
        assert_eq!(timer.is_done(), true);
        match timer.wait() {
            Ok(time_waited) => assert_eq!(time_waited, 2),
            Err(e) => assert_eq!(WaitError{}, e),
        }
        assert_eq!(timer.is_done(), true);
    }
}
