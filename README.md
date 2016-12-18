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
let waited = timer.wait();
assert_eq!(waited, 2);
assert_eq!(timer.is_done(), false);
```
