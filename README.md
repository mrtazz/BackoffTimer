# BackoffTimer

[![Build Status](https://travis-ci.org/mrtazz/BackoffTimer.svg?branch=master)](https://travis-ci.org/mrtazz/BackoffTimer)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)

## Overview

BackoffTimer is a timer written in rust that implements exponential backoff.

```rust
use backoff_timer::BackoffTimer;

let mut timer = BackoffTimer::new(5);
let waited = timer.wait();
assert_eq!(waited, 2);
assert_eq!(timer.is_done(), false);
```
