#!/usr/bin/env python3
"""
scripts/run_benchmarks.py — Real Empirical Performance Benchmark Suite for Gladeshell.

Measures sub-millisecond process latency and memory RSS using os.wait4, plus direct
Unix-socket Binary IPC round-trip microsecond latency, reporting min, p50, mean, p95,
p99, max, std_dev, and RSS across all supported shells and hot paths.
"""

from __future__ import annotations

import argparse
import contextlib
import json
import os
import platform
import shutil
import socket
import statistics as st
import struct
import subprocess as sp
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
BIN = str((REPO_ROOT / "target" / "release" / "gladeshell").resolve())


def percentile(sorted_vals: list[float], pct: float) -> float:
    """Compute linear-interpolated percentile (R-7) on a pre-sorted non-empty list."""
    if len(sorted_vals) == 1:
        return sorted_vals[0]
    pos = (pct / 100.0) * (len(sorted_vals) - 1)
    lo = int(pos)
    hi = min(lo + 1, len(sorted_vals) - 1)
    frac = pos - lo
    return sorted_vals[lo] + frac * (sorted_vals[hi] - sorted_vals[lo])


def summarize_samples(samples: list[float]) -> dict[str, float]:
    """Compute min, p50, mean, p95, p99, max, and stdev for latency samples."""
    ordered = sorted(samples)
    return {
        "count": len(ordered),
        "min": ordered[0],
        "p50": percentile(ordered, 50.0),
        "mean": st.mean(ordered),
        "p95": percentile(ordered, 95.0),
        "p99": percentile(ordered, 99.0),
        "max": ordered[-1],
        "std_dev": st.stdev(ordered) if len(ordered) > 1 else 0.0,
    }


def make_env(tmpdir: str, extra: dict[str, str] | None = None) -> dict[str, str]:
    """Build clean process environment pointing TMPDIR at tmpdir and BIN first on PATH."""
    env = os.environ.copy()
    bin_dir = str(Path(BIN).parent)
    env["PATH"] = f"{bin_dir}:{env.get('PATH', '')}"
    env["TMPDIR"] = tmpdir
    env["XDG_RUNTIME_DIR"] = tmpdir
    if extra:
        env.update(extra)
    return env


def measure_cmd(
    cmd_args: list[str],
    n_iterations: int = 100,
    warmup: int = 10,
    cwd: str | None = None,
    env: dict[str, str] | None = None,
) -> dict[str, Any] | None:
    """Benchmark a subprocess command via time.perf_counter_ns + os.wait4."""
    run_cwd = cwd or str(REPO_ROOT)
    for _ in range(warmup):
        try:
            proc = sp.Popen(
                cmd_args,
                stdout=sp.DEVNULL,
                stderr=sp.DEVNULL,
                cwd=run_cwd,
                env=env,
            )
            os.wait4(proc.pid, 0)
        except Exception:
            return None

    latencies_ms: list[float] = []
    max_rss_kb: list[int] = []

    for _ in range(n_iterations):
        t0 = time.perf_counter_ns()
        proc = sp.Popen(
            cmd_args,
            stdout=sp.DEVNULL,
            stderr=sp.DEVNULL,
            cwd=run_cwd,
            env=env,
        )
        _, _, ru = os.wait4(proc.pid, 0)
        t1 = time.perf_counter_ns()
        latencies_ms.append((t1 - t0) / 1e6)
        max_rss_kb.append(ru.ru_maxrss)

    stats = summarize_samples(latencies_ms)
    stats["max_rss_mb"] = max(max_rss_kb) / 1024.0
    stats["median_rss_mb"] = st.median(max_rss_kb) / 1024.0
    stats["min_rss_mb"] = min(max_rss_kb) / 1024.0
    return stats


@contextlib.contextmanager
def managed_daemon(env: dict[str, str]):
    """Spawn an isolated `gladeshell serve` daemon and return its Unix socket path."""
    sock_proc = sp.run(
        [BIN, "socket-path"],
        capture_output=True,
        text=True,
        env=env,
        check=True,
    )
    sock_path = sock_proc.stdout.strip()
    if os.path.exists(sock_path):
        os.unlink(sock_path)

    daemon = sp.Popen(
        [BIN, "serve"],
        stdout=sp.DEVNULL,
        stderr=sp.DEVNULL,
        env=env,
        cwd=str(REPO_ROOT),
    )
    try:
        deadline = time.monotonic() + 5.0
        while time.monotonic() < deadline:
            if os.path.exists(sock_path):
                try:
                    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
                        s.settimeout(0.2)
                        s.connect(sock_path)
                        s.sendall(struct.pack("<I", 11) + b"SOCKET-PATH")
                        hdr = s.recv(4)
                        if len(hdr) == 4:
                            break
                except OSError:
                    pass
            time.sleep(0.02)
        else:
            raise RuntimeError(f"Daemon failed to bind socket at {sock_path}")

        yield sock_path
    finally:
        daemon.terminate()
        try:
            daemon.wait(timeout=3.0)
        except sp.TimeoutExpired:
            daemon.kill()
            daemon.wait()
        if os.path.exists(sock_path):
            with contextlib.suppress(OSError):
                os.unlink(sock_path)


def recv_exact(sock: socket.socket, n_bytes: int) -> bytes:
    """Receive exactly n_bytes from a stream socket."""
    chunks = []
    remaining = n_bytes
    while remaining > 0:
        chunk = sock.recv(remaining)
        if not chunk:
            raise EOFError("unexpected socket EOF")
        chunks.append(chunk)
        remaining -= len(chunk)
    return b"".join(chunks)


def measure_ipc_roundtrip(
    sock_path: str,
    payload: bytes,
    n_iterations: int = 500,
    warmup: int = 25,
) -> dict[str, float]:
    """Measure packed [u32-LE len][payload] Unix socket round-trips in microseconds."""
    frame = struct.pack("<I", len(payload)) + payload

    def one_call() -> None:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
            s.connect(sock_path)
            s.sendall(frame)
            resp_len = struct.unpack("<I", recv_exact(s, 4))[0]
            if resp_len > 0:
                recv_exact(s, resp_len)

    for _ in range(warmup):
        one_call()

    latencies_us: list[float] = []
    for _ in range(n_iterations):
        t0 = time.perf_counter_ns()
        one_call()
        t1 = time.perf_counter_ns()
        latencies_us.append((t1 - t0) / 1e3)

    stats = summarize_samples(latencies_us)
    stats["req_per_sec"] = (1_000_000.0 / stats["mean"]) if stats["mean"] > 0 else 0.0
    return stats


def collect_metadata() -> dict[str, Any]:
    """Gather OS, CPU, Rust, binary, and installed shell metadata."""
    cpu_model = platform.processor() or "unknown"
    try:
        for line in Path("/proc/cpuinfo").read_text(encoding="utf-8").splitlines():
            if line.startswith("model name"):
                cpu_model = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass

    def cmd_ver(args: list[str]) -> str:
        try:
            res = sp.run(args, capture_output=True, text=True, timeout=3)
            if res.returncode == 0 and res.stdout.strip():
                return res.stdout.strip().splitlines()[0]
        except Exception:
            pass
        return "not installed"

    git_head = cmd_ver(["git", "rev-parse", "--short", "HEAD"])
    return {
        "timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "kernel": f"{platform.system()} {platform.release()} {platform.machine()}",
        "cpu": cpu_model,
        "cores_logical": os.cpu_count(),
        "python": platform.python_version(),
        "rustc": cmd_ver(["rustc", "--version"]),
        "binary": BIN,
        "binary_version": cmd_ver([BIN, "--version"]),
        "binary_size_bytes": Path(BIN).stat().st_size,
        "git_head": git_head,
        "shells": {
            "bash": cmd_ver(["bash", "--version"]) if shutil.which("bash") else "not installed",
            "brush": cmd_ver(["brush", "--version"]) if shutil.which("brush") else "not installed",
            "zsh": cmd_ver(["zsh", "--version"]) if shutil.which("zsh") else "not installed",
            "fish": cmd_ver(["fish", "--version"]) if shutil.which("fish") else "not installed",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Gladeshell empirical performance benchmark runner")
    parser.add_argument("-n", "--iterations", type=int, default=100, help="Subprocess iterations (default: 100)")
    parser.add_argument("--warmup", type=int, default=10, help="Subprocess warmup iterations (default: 10)")
    parser.add_argument("--ipc-iterations", type=int, default=500, help="IPC socket iterations (default: 500)")
    parser.add_argument("--json-out", type=str, default="", help="Optional path to write full JSON output")
    args = parser.parse_args()

    if not Path(BIN).exists():
        print(f"ERROR: Release binary not found at {BIN}. Run `cargo build --release` first.", file=sys.stderr)
        return 1

    meta = collect_metadata()
    print("=" * 96)
    print("⚡ Gladeshell Real Empirical Performance Benchmark (Python os.wait4 + Unix IPC Suite)")
    print("=" * 96)
    print(f"Binary   : {BIN} ({meta['binary_version']}, {meta['binary_size_bytes'] / (1024 * 1024):.2f} MiB, commit {meta['git_head']})")
    print(f"System   : {meta['kernel']} | {meta['cpu']} ({meta['cores_logical']} logical CPUs)")
    print(f"Samples  : {args.iterations} subprocess runs (+{args.warmup} warmup) | {args.ipc_iterations} IPC round-trips")

    # Populate shell init caches in real ~/.gladeshell/cache and wake CPU P-states cleanly.
    with tempfile.TemporaryDirectory(prefix="gladeshell_bench_no_sock_") as empty_tmp:
        off_env = make_env(empty_tmp, {"GLADESHELL_TERMINAL_INTEGRATION": "off"})
        osc_auto_env = make_env(
            empty_tmp,
            {"GLADESHELL_TERMINAL_INTEGRATION": "auto", "GHOSTTY_RESOURCES_DIR": "/tmp"},
        )
        for sh in ("bash", "brush", "zsh", "fish", "pwsh"):
            sp.run([BIN, "init", sh], stdout=sp.DEVNULL, stderr=sp.DEVNULL, env=off_env)
        for _ in range(25):
            sp.run([BIN, "version"], stdout=sp.DEVNULL, stderr=sp.DEVNULL, env=off_env)

        process_benchmarks: list[tuple[str, list[str], str, dict[str, str]]] = [
            ("gladeshell version (fast-path)", [BIN, "version"], str(REPO_ROOT), off_env),
            ("gladeshell prompt --transient", [BIN, "prompt", "--transient"], str(REPO_ROOT), off_env),
            ("gladeshell socket-path (fast-path)", [BIN, "socket-path"], str(REPO_ROOT), off_env),
            ("gladeshell z glade (frecent lookup)", [BIN, "z", "glade"], str(REPO_ROOT), off_env),
            ("gladeshell integration status (clap)", [BIN, "integration", "status"], str(REPO_ROOT), off_env),
            (
                "gladeshell prompt (/tmp, no daemon)",
                [BIN, "prompt", "--shell", "zsh", "--cwd", "/tmp", "--exit-code", "0", "--cmd-duration", "0"],
                "/tmp",
                off_env,
            ),
            (
                "gladeshell prompt (repo git, no daemon)",
                [BIN, "prompt", "--shell", "zsh", "--cwd", str(REPO_ROOT), "--exit-code", "0", "--cmd-duration", "0"],
                str(REPO_ROOT),
                off_env,
            ),
            ("gladeshell init bash (cold gen)", [BIN, "init", "bash"], str(REPO_ROOT), off_env),
            ("gladeshell init brush (cold gen)", [BIN, "init", "brush"], str(REPO_ROOT), off_env),
            ("gladeshell init zsh (cold gen)", [BIN, "init", "zsh"], str(REPO_ROOT), off_env),
            ("gladeshell init fish (cold gen)", [BIN, "init", "fish"], str(REPO_ROOT), off_env),
            ("gladeshell init pwsh (cold gen)", [BIN, "init", "pwsh"], str(REPO_ROOT), off_env),
        ]

        if shutil.which("bash"):
            process_benchmarks.append(
                (
                    "Bash hook source (cached, OSC off)",
                    ["bash", "--norc", "--noprofile", "-c", "source ~/.gladeshell/cache/init.bash"],
                    str(REPO_ROOT),
                    off_env,
                )
            )
            process_benchmarks.append(
                (
                    "Bash hook source (cached, OSC auto)",
                    ["bash", "--norc", "--noprofile", "-c", "source ~/.gladeshell/cache/init.bash"],
                    str(REPO_ROOT),
                    osc_auto_env,
                )
            )
        if shutil.which("brush"):
            process_benchmarks.append(
                (
                    "Brush hook source (cached, OSC off)",
                    ["brush", "--norc", "--noprofile", "--no-config", "-c", "source ~/.gladeshell/cache/init.brush"],
                    str(REPO_ROOT),
                    off_env,
                )
            )
            process_benchmarks.append(
                (
                    "Brush hook source (cached, OSC auto)",
                    ["brush", "--norc", "--noprofile", "--no-config", "-c", "source ~/.gladeshell/cache/init.brush"],
                    str(REPO_ROOT),
                    osc_auto_env,
                )
            )
        if shutil.which("zsh"):
            process_benchmarks.append(
                (
                    "Zsh hook source (cached, OSC off)",
                    ["zsh", "--no-rcs", "-c", "source ~/.gladeshell/cache/init.zsh"],
                    str(REPO_ROOT),
                    off_env,
                )
            )
            process_benchmarks.append(
                (
                    "Zsh hook source (cached, OSC auto)",
                    ["zsh", "--no-rcs", "-c", "source ~/.gladeshell/cache/init.zsh"],
                    str(REPO_ROOT),
                    osc_auto_env,
                )
            )
        if shutil.which("fish"):
            process_benchmarks.append(
                (
                    "Fish hook source (cached, OSC off)",
                    ["fish", "--no-config", "-c", "source ~/.gladeshell/cache/init.fish"],
                    str(REPO_ROOT),
                    off_env,
                )
            )

        proc_results: dict[str, Any] = {}
        header = (
            f"\n{'Benchmark Target':<40} | {'Min':>7} | {'p50':>7} | {'Mean':>7} | "
            f"{'p95':>7} | {'p99':>7} | {'Max':>7} | {'MaxRSS':>8}"
        )
        print(header)
        print("-" * len(header.strip()))

        for label, cmd, run_cwd, run_env in process_benchmarks:
            data = measure_cmd(cmd, n_iterations=args.iterations, warmup=args.warmup, cwd=run_cwd, env=run_env)
            if data:
                proc_results[label] = data
                print(
                    f"{label:<40} | {data['min']:>5.2f}ms | {data['p50']:>5.2f}ms | {data['mean']:>5.2f}ms | "
                    f"{data['p95']:>5.2f}ms | {data['p99']:>5.2f}ms | {data['max']:>5.2f}ms | {data['max_rss_mb']:>6.2f}MB"
                )
            else:
                print(f"{label:<40} | SKIPPED / FAILED")

    # Daemon-backed CLI prompt + direct Binary Unix IPC benchmarks
    ipc_results: dict[str, Any] = {}
    with tempfile.TemporaryDirectory(prefix="gladeshell_bench_ipc_") as ipc_tmp:
        daemon_env = make_env(ipc_tmp, {"GLADESHELL_TERMINAL_INTEGRATION": "off"})
        with managed_daemon(daemon_env) as sock_path:
            # 1) CLI prompt hitting live daemon socket
            for label, cmd, run_cwd in [
                (
                    "gladeshell prompt (/tmp, daemon CLI)",
                    [BIN, "prompt", "--shell", "zsh", "--cwd", "/tmp", "--exit-code", "0", "--cmd-duration", "0"],
                    "/tmp",
                ),
                (
                    "gladeshell prompt (repo git, daemon CLI)",
                    [BIN, "prompt", "--shell", "zsh", "--cwd", str(REPO_ROOT), "--exit-code", "0", "--cmd-duration", "0"],
                    str(REPO_ROOT),
                ),
            ]:
                data = measure_cmd(cmd, n_iterations=args.iterations, warmup=args.warmup, cwd=run_cwd, env=daemon_env)
                if data:
                    proc_results[label] = data
                    print(
                        f"{label:<40} | {data['min']:>5.2f}ms | {data['p50']:>5.2f}ms | {data['mean']:>5.2f}ms | "
                        f"{data['p95']:>5.2f}ms | {data['p99']:>5.2f}ms | {data['max']:>5.2f}ms | {data['max_rss_mb']:>6.2f}MB"
                    )

            # 2) Direct Binary Unix Socket IPC round-trip (microseconds)
            ipc_targets = [
                ("IPC Prompt (/tmp, clean)", f"/tmp\x1f0\x1f0\x1fuser\x1fhost\x1f0\x1f0".encode()),
                ("IPC Prompt (repo git, TTL warm)", f"{REPO_ROOT}\x1f0\x1f0\x1fuser\x1fhost\x1f0\x1f0".encode()),
                ("IPC Autosuggest ('git st')", b"SUGGEST\x1fgit st"),
                ("IPC Highlight ('git status && ls')", b"HIGHLIGHT\x1fgit status && ls"),
            ]
            ipc_hdr = (
                f"\n{'Binary IPC Target (UnixSocket)':<40} | {'Min':>7} | {'p50':>7} | {'Mean':>7} | "
                f"{'p95':>7} | {'p99':>7} | {'Max':>7} | {'Req/s':>9}"
            )
            print(ipc_hdr)
            print("-" * len(ipc_hdr.strip()))
            for label, payload in ipc_targets:
                # Warm repo cache once cleanly inside 25 warmups
                stats = measure_ipc_roundtrip(sock_path, payload, n_iterations=args.ipc_iterations, warmup=25)
                ipc_results[label] = stats
                print(
                    f"{label:<40} | {stats['min']:>5.1f}µs | {stats['p50']:>5.1f}µs | {stats['mean']:>5.1f}µs | "
                    f"{stats['p95']:>5.1f}µs | {stats['p99']:>5.1f}µs | {stats['max']:>5.1f}µs | {stats['req_per_sec']:>9.0f}"
                )

    full_report = {
        "metadata": meta,
        "process_benchmarks_ms": proc_results,
        "ipc_benchmarks_us": ipc_results,
    }
    if args.json_out:
        Path(args.json_out).write_text(json.dumps(full_report, indent=2), encoding="utf-8")
        print(f"\nSaved full JSON report to {args.json_out}")
    else:
        print("\n" + "=" * 96)
        print("JSON_REPORT_START")
        print(json.dumps(full_report, indent=2))
        print("JSON_REPORT_END")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
