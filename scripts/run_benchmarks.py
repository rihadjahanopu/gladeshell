#!/usr/bin/env python3
"""
scripts/run_benchmarks.py — Full Empirical Benchmark Runner for Gladeshell
Measures sub-millisecond process latency and memory RSS using os.wait4 across
50 iterations on Linux.
"""

import os
import subprocess as sp
import time
import statistics as st
import shutil
import json

BIN = os.path.abspath("./target/release/gladeshell")

def measure_cmd(cmd_args, n_iterations=50, warmup=5):
    # Warmup
    for _ in range(warmup):
        try:
            p = sp.Popen(cmd_args, stdout=sp.DEVNULL, stderr=sp.DEVNULL)
            os.wait4(p.pid, 0)
        except Exception as e:
            return None

    latencies_ms = []
    max_rss_kb = []

    for _ in range(n_iterations):
        t0 = time.perf_counter_ns()
        p = sp.Popen(cmd_args, stdout=sp.DEVNULL, stderr=sp.DEVNULL)
        _, _, ru = os.wait4(p.pid, 0)
        t1 = time.perf_counter_ns()
        latencies_ms.append((t1 - t0) / 1e6)
        max_rss_kb.append(ru.ru_maxrss)

    latencies_ms.sort()
    p95_idx = int(0.95 * len(latencies_ms))

    return {
        "mean": st.mean(latencies_ms),
        "median": st.median(latencies_ms),
        "min": min(latencies_ms),
        "p95": latencies_ms[p95_idx],
        "std_dev": st.stdev(latencies_ms) if len(latencies_ms) > 1 else 0.0,
        "max_rss_mb": max(max_rss_kb) / 1024.0,
        "min_rss_mb": min(max_rss_kb) / 1024.0,
    }

def main():
    print("=" * 70)
    print("⚡ Gladeshell Empirical Performance Benchmark (Python os.wait4 Suite)")
    print("=" * 70)
    print(f"Binary: {BIN}")

    # Ensure cache exists
    sp.run([BIN, "init", "bash"], stdout=sp.DEVNULL, stderr=sp.DEVNULL)
    sp.run([BIN, "init", "zsh"], stdout=sp.DEVNULL, stderr=sp.DEVNULL)
    sp.run([BIN, "init", "fish"], stdout=sp.DEVNULL, stderr=sp.DEVNULL)

    benchmarks = [
        ("gladeshell prompt (warm)", [BIN, "prompt"]),
        ("gladeshell prompt --transient", [BIN, "prompt", "--transient"]),
        ("gladeshell version (fast-path)", [BIN, "version"]),
        ("gladeshell z glade (frecent)", [BIN, "z", "glade"]),
        ("gladeshell init bash (cold gen)", [BIN, "init", "bash"]),
        ("gladeshell init zsh (cold gen)", [BIN, "init", "zsh"]),
        ("gladeshell init fish (cold gen)", [BIN, "init", "fish"]),
    ]

    if shutil.which("bash"):
        benchmarks.append(("Bash hook source (cached)", ["bash", "--norc", "-c", "source ~/.gladeshell/cache/init.bash"]))
    if shutil.which("zsh"):
        benchmarks.append(("Zsh hook source (cached)", ["zsh", "--no-rcs", "-c", "source ~/.gladeshell/cache/init.zsh"]))
    if shutil.which("fish"):
        benchmarks.append(("Fish hook source (cached)", ["fish", "--no-config", "-c", "source ~/.gladeshell/cache/init.fish"]))

    results = {}

    print(f"\nRunning 50 iterations (+ 5 warmup) per benchmark...\n")
    print(f"{'Benchmark Target':<35} | {'Mean':>8} | {'Median':>8} | {'Min':>8} | {'p95':>8} | {'Max RSS':>8}")
    print("-" * 88)

    for name, cmd in benchmarks:
        data = measure_cmd(cmd, n_iterations=50, warmup=5)
        if data:
            results[name] = data
            print(f"{name:<35} | {data['mean']:>6.2f}ms | {data['median']:>6.2f}ms | {data['min']:>6.2f}ms | {data['p95']:>6.2f}ms | {data['max_rss_mb']:>6.2f}MB")
        else:
            print(f"{name:<35} | SKIPPED / FAILED")

    print("\n" + "=" * 70)
    print("Benchmark complete. JSON dump:")
    print(json.dumps(results, indent=2))

if __name__ == "__main__":
    main()
