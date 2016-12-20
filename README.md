# BackoffTimer

[![Build Status](https://travis-ci.org/mrtazz/BackoffTimer.svg?branch=master)](https://travis-ci.org/mrtazz/BackoffTimer)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)

## Overview

BackoffTimer is a rust library that implements different algorithms for
backoff timers.


```rust
use backoff_timer::BackoffTimer;
use backoff_timer::ExponentialBackoffTimer;

/// exponentially back off and wait at most 5 seconds
let mut timer = ExponentialBackoffTimer::new(5);
let mut timer = ExponentialBackoffTimer::new(6);
match timer.wait() {
    Ok(time_waited) => assert_eq!(time_waited, 2),
    Err(e) => panic!(e),
}
assert_eq!(timer.is_done(), false);
```
