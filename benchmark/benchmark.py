"""
Benchmark intervaltree_rs vs intervaltree (PyPI).

- intervaltree (PyPI): IntervalTree.from_tuples(...), query via .overlap(l, r)
- intervaltree_rs: IntervalTree(), .add(l, r, data) or .insert(l, r, data), optional .build(), query via .search/.overlap
Adapt below if your API differs.
"""

import gc
import math
import os
import random
import time
from dataclasses import dataclass
from typing import Any, Callable, List, Tuple

import matplotlib.pyplot as plt

# ---------------------- Config ----------------------
SEED = 123
SIZES = [1_000, 10_000, 100_000]       # bump as you like (careful with RAM/time)
N_QUERIES = 5_000
MAX_COORD = 10_000_000
AVG_LEN = 500                          # avg interval length (exp-like)
QUERY_WIDTHS = [1, 10_000, 100_000, 1_000_000]  # small/med/large searches
REPEATS = 3                            # best-of repeats for timing
OUTDIR = "bench_plots"
# ----------------------------------------------------

random.seed(SEED)

Interval = Tuple[int, int, Any]
Query = Tuple[int, int]

def gen_intervals(n: int, max_coord: int, avg_len: int) -> List[Interval]:
    """Random intervals with exponential-ish lengths, clamped to [0, max_coord]."""
    out: List[Interval] = []
    for _ in range(n):
        l = random.randrange(0, max_coord)
        length = max(1, int(random.expovariate(1.0 / avg_len)))
        r = min(max_coord, l + length)
        if r == l:
            r = min(max_coord, l + 1)
        out.append((l, r, None))
    return out

def gen_queries(nq: int, max_coord: int, width: int) -> List[Query]:
    """Queries of fixed width, clamped within [0, max_coord]."""
    width = max(1, width)
    start_max = max(0, max_coord - width)
    qs: List[Query] = []
    for _ in range(nq):
        # if width > max_coord, start_max==0 -> always l=0, r=max_coord
        l = random.randrange(0, max(1, start_max))
        r = min(max_coord, l + width)
        qs.append((l, r))
    return qs

# ---------------------- Adapters ----------------------
@dataclass
class Impl:
    name: str
    build: Callable[[List[Interval]], Any]    # returns a tree
    query: Callable[[Any, int, int], int]     # returns hit count

def _mk_intervaltree_py() -> Impl:
    # import here so missing dependency doesn't crash whole script
    from intervaltree import IntervalTree  # type: ignore

    def build(intervals: List[Interval]) -> IntervalTree:
        # one-shot construction is faster than incremental adds
        return IntervalTree.from_tuples([(l, r, d) for (l, r, d) in intervals])

    def query(tree: Any, l: int, r: int) -> int:
        return len(tree.overlap(l, r))

    return Impl("intervaltree (PyPI)", build, query)

def _mk_intervaltree_rs() -> Impl:
    import intervaltree_rs as itrs  # type: ignore

    def build(intervals: List[Interval]) -> Any:
        tree = getattr(itrs, "IntervalTree")()
        add = getattr(tree, "add", None) or getattr(tree, "insert", None)
        if add is None:
            raise RuntimeError("intervaltree_rs: no add/insert method found")
        for (l, r, d) in intervals:
            # pass as three args, not a tuple
            add((l, r, d))
        if hasattr(tree, "build"):
            tree.build()
        return tree

    def query(tree: Any, l: int, r: int) -> int:
        if hasattr(tree, "search"):
            res = tree.search(l, r)
        elif hasattr(tree, "overlap"):
            res = tree.overlap(l, r)
        else:
            raise RuntimeError("intervaltree_rs: no search/overlap method found")
        # Prefer len if available; otherwise iterate to count
        try:
            return len(res)
        except TypeError:
            return sum(1 for _ in res)

    return Impl("intervaltree_rs", build, query)

ADAPTERS: List[Impl] = []
try:
    ADAPTERS.append(_mk_intervaltree_rs())
except Exception as e:
    print(f"[warn] Skipping intervaltree_rs: {e}")
try:
    ADAPTERS.append(_mk_intervaltree_py())
except Exception as e:
    print(f"[warn] Skipping intervaltree (PyPI): {e}")

if len(ADAPTERS) < 2:
    print("[error] Need both implementations to compare. Install both packages and rerun.")
    # Script still runs and saves plots for whatever is available.

# ---------------------- Timing helpers ----------------------
def time_best_of(repeats: int, fn: Callable[[], Any]) -> float:
    best = math.inf
    for _ in range(repeats):
        gc.collect()
        t0 = time.perf_counter()
        fn()
        dt = time.perf_counter() - t0
        if dt < best:
            best = dt
    return best

# ---------------------- Benchmark ----------------------
build_times = {impl.name: [] for impl in ADAPTERS}
query_times = {impl.name: {w: [] for w in QUERY_WIDTHS} for impl in ADAPTERS}
query_throughput = {impl.name: {w: [] for w in QUERY_WIDTHS} for impl in ADAPTERS}

for n in SIZES:
    print(f"\n=== N={n} ===")
    intervals = gen_intervals(n, MAX_COORD, AVG_LEN)

    # Pre-generate queries for each width (same set across impls)
    queries_by_w = {w: gen_queries(N_QUERIES, MAX_COORD, w) for w in QUERY_WIDTHS}

    for impl in ADAPTERS:
        # Measure build (object dropped each repeat), then build once to keep
        try:
            t_build = time_best_of(REPEATS, lambda: impl.build(intervals))
            tree = impl.build(intervals)
        except MemoryError:
            t_build = math.nan
            tree = None
        except Exception as e:
            print(f"[warn] {impl.name}: build failed: {e}")
            t_build = math.nan
            tree = None

        build_times[impl.name].append(t_build)
        print(f"{impl.name}: build {t_build:.4f}s")

        if tree is None:
            for w in QUERY_WIDTHS:
                query_times[impl.name][w].append(math.nan)
                query_throughput[impl.name][w].append(math.nan)
            continue

        # Batch search timing
        for w, queries in queries_by_w.items():
            def run_queries() -> int:
                total = 0
                for (l, r) in queries:
                    total += impl.query(tree, l, r)
                return total

            t_query = time_best_of(REPEATS, run_queries)
            qps = (N_QUERIES / t_query) if t_query > 0 else float("inf")
            query_times[impl.name][w].append(t_query)
            query_throughput[impl.name][w].append(qps)
            print(f"{impl.name}: search width={w} total {t_query:.4f}s ({qps:.0f} q/s)")

# ---------------------- Save plots ----------------------
os.makedirs(OUTDIR, exist_ok=True)

# Build time vs N
plt.figure()
for impl in ADAPTERS:
    plt.plot(SIZES, build_times[impl.name], marker="o", label=impl.name)
plt.xlabel("Number of intervals (N)")
plt.ylabel("Build time (s)")
plt.title("Build time vs N")
plt.legend()
plt.tight_layout()
plt.savefig(os.path.join(OUTDIR, "build_time_vs_N.png"), dpi=200)
plt.close()

# Total search time vs N for each query width
for w in QUERY_WIDTHS:
    plt.figure()
    for impl in ADAPTERS:
        plt.plot(SIZES, query_times[impl.name][w], marker="o", label=impl.name)
    plt.xlabel("Number of intervals (N)")
    plt.ylabel(f"Total time for {N_QUERIES} queries (s)")
    plt.title(f"Search time vs N (query width={w})")
    plt.legend()
    plt.tight_layout()
    plt.savefig(os.path.join(OUTDIR, f"search_time_vs_N_w{w}.png"), dpi=200)
    plt.close()

# Throughput vs N (higher is better)
for w in QUERY_WIDTHS:
    plt.figure()
    for impl in ADAPTERS:
        plt.plot(SIZES, query_throughput[impl.name][w], marker="o", label=impl.name)
    plt.xlabel("Number of intervals (N)")
    plt.ylabel("Throughput (queries/sec)")
    plt.title(f"Search throughput vs N (query width={w})")
    plt.legend()
    plt.tight_layout()
    plt.savefig(os.path.join(OUTDIR, f"throughput_vs_N_w{w}.png"), dpi=200)
    plt.close()

print(f"✅ All plots saved under ./{OUTDIR}/")
