# HashMap vs DashMap Benchmark

A simple benchmark comparing the performance of `HashMap` and `DashMap` for basic operations.

## Results

### Insert Performance (1000 items)
- HashMap: ~40.3 µs
- DashMap: ~49.0 µs
- Result: HashMap is ~20% faster for inserts

### Read Performance (1000 items)
- HashMap: ~4.7 µs
- DashMap: ~13.2 µs
- Result: HashMap is ~3x faster for reads

## Key Takeaways

- **Use HashMap when:**
  - You only need single-thread access
  - Performance is critical
  - Thread safety isn't required

- **Use DashMap when:**
  - You need thread safety (multiple threads)
  - Concurrent access is required
  - Willing to trade some performance for thread safety

## Performance Notes

- HashMap advantages:
  - 20% faster for inserts
  - 3x faster for reads
  - Lower memory overhead
  - But isn't thread-safe

- Benchmark variations may occur due to:
  - System background tasks
  - CPU throttling
  - Memory operations

## Technical Details
- Benchmark tool: Criterion.rs
- Each operation tested with 100 samples
- Tests run on 1000 integer key-value pairs
- All measurements in microseconds (µs)