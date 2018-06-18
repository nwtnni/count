# count

A macro for generating thread-local global counters. Useful for simple ID generation.

## Usage

```rust
#[macro_use]
extern crate count;

generate_counter!(Counter, usize);

fn main() {

  // Starts at 0 by default
  assert_eq!(Counter::next(), 0);
  assert_eq!(Counter::next(), 1);
  assert_eq!(Counter::next(), 2);

  // Can be set to arbitrary value
  Counter::set(1000);
  assert_eq!(Counter::next(), 1000);
  assert_eq!(Counter::next(), 1001);
  assert_eq!(Counter::next(), 1002);

  // Or reset to 0
  Counter::reset();
  assert_eq!(Counter::next(), 0);
}
```

## Implementation

Creates a new module with a thread-local static `Cell`.
