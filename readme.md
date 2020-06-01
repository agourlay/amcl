# amcl

[![Build Status](https://travis-ci.org/agourlay/amcl.svg?branch=master)](https://travis-ci.org/agourlay/amcl)

This library implements `Additive/Multiplicative Control Loops`.

The most common flavour being the [Additive increase/multiplicative decrease (AIMD)](https://en.wikipedia.org/wiki/Additive_increase/multiplicative_decrease) best known for its use in TCP congestion control.

`AIMD` combines linear growth of the congestion window with an exponential reduction when congestion is detected. The result is a saw-tooth behavior that represents the probe for bandwidth.

For completeness the library proposes as well `MIMD`, `AIAD` & `MIAD` although they are less useful.

Disclaimer: The core logic of this library is extremely simple and serves mostly as an exercise to build a Rust library.

## usage

A controller updates a value called `current` following calls to `fn update(bool)`.

Example of an `AIMD` controller increasing by one on success and decreasing by half on failure.

```rust
   use amcl::Controller;

   let mut aimd = Controller::aimd(1.0, 2.0).unwrap();
    for i in 0..=100 {
        if i == 50 {
            println!("congestion detected, going down by half");
            aimd.update(false);
        } else {
            aimd.update(true);
        }
    }
    println!("{}", aimd.current()) // prints '75'
```

It is possible to set the initial value, a min value and a max value for the controller's value.