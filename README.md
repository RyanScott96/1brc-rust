# One Billion Row Challenge (1BRC) - Rust

A highly concurrent, lock-free data ingestion engine written in Rust, designed to parse and aggregate one billion rows of weather station data in under 10 seconds. 

This project was built to explore bare-metal hardware constraints, memory layouts, and multi-core CPU saturation without relying on thread-blocking mutexes.

## 🚀 Architecture & Performance

This pipeline achieves sub-10-second execution times for a 13GB dataset on consumer hardware by aggressively minimizing allocations and OS overhead:

* **Memory Mapping:** Utilizes `memmap2` to map the entire 13GB file directly into RAM, bypassing traditional buffered I/O overhead.
* **Lock-Free Concurrency:** Implements a Map-Reduce architecture using `crossbeam` scoped threads. Each thread operates on its own chunk of the memory map and aggregates data into a thread-local hash map, resulting in zero lock contention on the hot path.
* **Zero-Copy Parsing:** Operates entirely on raw `&[u8]` byte slices. String allocations are delayed until the final stdout print phase.
* **Custom State Machine:** Replaces heavy regex or string-splitting with a custom, byte-by-byte state machine to identify delimiters and parse temperature floats as fast integers.

## 🛠️ Build and Run

**1. Generate the Data**
You need the 1 billion row `measurements.txt` file. Instructions are found at https://github.com/gunnarmorling/1brc

**2. Build the binary**
```bash
cargo build --release
```

**3. Run and Time**
```
bash
time ./target/release/onebrc
```