#!/bin/bash
# Performance benchmark script for hybrid architecture
set -e

# Run Rust criterion benchmarks if available
if cargo bench --help | grep -q criterion; then
  echo "Running Rust criterion benchmarks..."
  cargo bench --all --bench '*'
else
  echo "No criterion benches found, running default benches..."
  cargo bench || true
fi

# Optionally, run custom performance tests (expand as needed)
# Example: ./scripts/custom_perf_test.sh

# Collect and print summary
if [ -d target/criterion ]; then
  echo "Benchmark results:"
  find target/criterion -name 'report/index.html' || true
fi
