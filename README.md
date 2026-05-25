# Lav-Seed

Lav-Seed is a deterministic numeric ID generator based on permutation techniques. It generates unique IDs within a defined range without collisions, making it suitable for systems that require predictable and fast identifier generation.

## Features

* Deterministic ID generation
* Collision-free within configured range
* Fast O(1) generation
* Configurable range and seed key
* Custom transform support

## Installation

Add this to your `Cargo.toml`:

```toml
lav-seed = "0.1"
```

## Usage

### Basic example

```rust
use lav_seed::Generator;

let mut gen = Generator::new(0)
    .min_seed(1)
    .max_seed(1_000_000)
    .key(12345)
    .build()
    .unwrap();

let id = gen.generate().unwrap();
println!("{}", id);
```

### Using ConfigBuilder

```rust
use lav_seed::{ConfigBuilder, Generator};

let config = ConfigBuilder::new()
    .min_seed(100)
    .max_seed(1_000_000)
    .key(42)
    .initial_counter(0)
    .build()
    .unwrap();

let mut gen = Generator::from_config(config);

let id = gen.generate().unwrap();
println!("{}", id);
```

## How it works

Lav-Seed uses a permutation-based approach:

```
f(x) = (a * x + b) mod n
```

Where:

* x is the internal counter
* a is a fixed multiplier
* b is the key (seed)
* n is the size of the range

This ensures that each number in the range is visited exactly once before repeating.

## Use cases

* User ID generation
* Referral codes
* Game item IDs
* Distributed systems
* Temporary identifiers
