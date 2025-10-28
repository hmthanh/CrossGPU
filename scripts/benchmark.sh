#!/usr/bin/env bash
# Run comprehensive benchmarks across all backends

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RESULTS_DIR="$PROJECT_ROOT/benchmark-results"

mkdir -p "$RESULTS_DIR"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="$RESULTS_DIR/benchmark_$TIMESTAMP.json"

echo "🔬 Running CrossGPU Benchmarks"
echo "================================"
echo ""

# Build in release mode
echo "⚙️  Building in release mode..."
cargo build --release --all
echo ""

# Create benchmark results
cat > "$RESULTS_FILE" <<EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "platform": "$(uname -s)",
  "arch": "$(uname -m)",
  "rust_version": "$(rustc --version)",
  "benchmarks": [
EOF

# Function to run benchmark
run_benchmark() {
    local name=$1
    local command=$2

    echo "🏃 Running: $name"

    # Run 5 iterations
    local times=()
    for i in {1..5}; do
        echo -n "  Iteration $i/5... "
        start=$(date +%s%N)
        eval "$command" > /dev/null 2>&1 || true
        end=$(date +%s%N)
        elapsed=$((($end - $start) / 1000000)) # Convert to milliseconds
        times+=($elapsed)
        echo "${elapsed}ms"
    done

    # Calculate average
    local sum=0
    for t in "${times[@]}"; do
        sum=$((sum + t))
    done
    local avg=$((sum / ${#times[@]}))

    # Calculate min/max
    local min=${times[0]}
    local max=${times[0]}
    for t in "${times[@]}"; do
        ((t < min)) && min=$t
        ((t > max)) && max=$t
    done

    echo "  Average: ${avg}ms (min: ${min}ms, max: ${max}ms)"
    echo ""

    # Append to results
    cat >> "$RESULTS_FILE" <<EOF
    {
      "name": "$name",
      "iterations": ${#times[@]},
      "times_ms": [$(IFS=,; echo "${times[*]}")],
      "avg_ms": $avg,
      "min_ms": $min,
      "max_ms": $max
    },
EOF
}

# Run benchmarks
run_benchmark "cargo test (all)" "cargo test --release --all"
run_benchmark "simple-inference" "./target/release/simple-inference"
run_benchmark "cargo check (all)" "cargo check --all"
run_benchmark "cargo build (debug)" "cargo build --all"

# Close JSON
sed -i.bak '$ s/,$//' "$RESULTS_FILE"  # Remove trailing comma
rm -f "${RESULTS_FILE}.bak"
cat >> "$RESULTS_FILE" <<EOF
  ]
}
EOF

echo "================================"
echo "✅ Benchmarks complete!"
echo ""
echo "📊 Results saved to: $RESULTS_FILE"
echo ""

# Generate summary report
cat > "$RESULTS_DIR/benchmark_${TIMESTAMP}.md" <<EOF
# CrossGPU Benchmark Results

**Date**: $(date)
**Platform**: $(uname -s) $(uname -m)
**Rust**: $(rustc --version)

## Results

| Benchmark | Average | Min | Max |
|-----------|---------|-----|-----|
EOF

# Parse JSON and create table
jq -r '.benchmarks[] | "| \(.name) | \(.avg_ms)ms | \(.min_ms)ms | \(.max_ms)ms |"' "$RESULTS_FILE" >> "$RESULTS_DIR/benchmark_${TIMESTAMP}.md"

cat >> "$RESULTS_DIR/benchmark_${TIMESTAMP}.md" <<EOF

## System Information

\`\`\`
$(uname -a)
\`\`\`

## Environment

\`\`\`
Rust: $(rustc --version)
Cargo: $(cargo --version)
\`\`\`

## Raw Data

See \`benchmark_${TIMESTAMP}.json\` for detailed timing data.
EOF

echo "📄 Markdown report: $RESULTS_DIR/benchmark_${TIMESTAMP}.md"
echo ""

# Display summary
if command -v jq &> /dev/null; then
    echo "📈 Summary:"
    jq -r '.benchmarks[] | "  \(.name): \(.avg_ms)ms"' "$RESULTS_FILE"
else
    echo "⚠️  Install 'jq' for formatted output"
fi

echo ""
echo "💡 Tips:"
echo "   - Compare results over time to track performance"
echo "   - Run on different hardware to compare backends"
echo "   - Use 'cargo flamegraph' for detailed profiling"
echo "   - Use 'cargo bench' for micro-benchmarks (add #[bench] tests)"
