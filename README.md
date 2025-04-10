# BloomFilter-RS

A space-efficient probabilistic data structure implemented in Rust for membership testing.

## Features

- Space-efficient implementation using `BitVec` (8x memory savings compared to `Vec<bool>`)
- Multiple hash algorithms (Default, MD5, SHA1, SHA256)
- Error handling with custom error types
- Comprehensive test coverage
- Detailed documentation with examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bloomfilter-rs = { git = "https://github.com/你的用户名/bloomfilter-rs" }
```

## Usage

```rust
use bloomfilter_rs::BloomFilter;

// Create a new Bloom filter with 1000 buckets and 4 hash functions
let mut bf = BloomFilter::new(1000, 4).unwrap();

// Insert elements
bf.insert(&"test");
bf.insert(&42);

// Check if elements exist
assert!(bf.check(&"test")); // true
assert!(bf.check(&42)); // true
assert!(!bf.check(&"not_in_set")); // false

// Get error rate
let error_rate = bf.error_chance();
println!("False positive probability: {}", error_rate);

// Clear the filter
bf.clear();
assert!(bf.is_empty());
```

# Example
```shell
cargo run --example basic_usage
cargo run --example advanced_usage
```

## API

### `BloomFilter::new(size: usize, hash_count: usize) -> Result<BloomFilter, BloomFilterError>`

Creates a new Bloom filter with the specified size and number of hash functions.

### `insert<T>(&mut self, word: &T) where T: Hash`

Inserts an element into the Bloom filter.

### `check<T>(&self, word: &T) -> bool where T: Hash`

Checks if an element is possibly in the set.

### `error_chance(&self) -> f32`

Calculates the probability of a false positive.

### `clear(&mut self)`

Clears all elements from the Bloom filter.

### `capacity(&self) -> usize`

Returns the capacity of the Bloom filter (number of buckets).

### `len(&self) -> u64`

Returns the number of elements inserted into the Bloom filter.

### `is_empty(&self) -> bool`

Checks if the Bloom filter is empty.

## Error Handling

The library provides custom error types:

```rust
pub enum BloomFilterError {
    InvalidSize,
    InvalidHashCount,
    Overflow,
}
```

## Performance

- Memory usage: 8x less than a standard boolean array
- Time complexity: O(k) for insert and check operations, where k is the number of hash functions

## License

MIT
