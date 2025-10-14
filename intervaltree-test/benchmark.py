"""
Benchmark intervaltree_rs vs intervaltree.

Assumptions:
- `intervaltree` is the PyPI library: `from intervaltree import IntervalTree`, query via `overlap(l, r)`.
- `intervaltree_rs` exposes: `IntervalTree()`, methods `.add(l, r, data=None)` or `.insert(l, r, data=None)`,
  optional `.build()`, and query via `.search(l, r)` or `.overlap(l, r)`.
Adapt below in ADAPTERS if your API differs.
"""

import math
import random
import time
from dataclasses import dataclass
from typing import Callable, Iterable, List, Tuple, Any
from intervaltree import IntervalTree
import intervaltree_rs as itrs  # type: ignore

import matplotlib.pyplot as plt

# ---------------------- Config ----------------------
SEED = 123
SIZES = [1_000, 10_000, 100_000, 300_000, 1_000_000]  # increase/decrease as you like
N_QUERIES = 5_000
MAX_COORD = 10_000_000
AVG_LEN = 500  # average interval length
QUERY_WIDTHS = [1, 10_000, 1_000_000]  # small / medium / large
REPEATS = 3  # timing repeats per point; best-of to reduce noise
# ----------------------------------------------------

random.seed(SEED)

Interval = Tuple[int, int, Any]
Query = Tuple[int, int]

def gen_intervals(n: int, max_coord: int, avg_len: int) -> List[Interval]:
    # Geometric-ish lengths, clamped within [0, max_coord]
    out: List[Interval] = []
    for _ in range(n):
        l = random.randrange(0, max_coord)
        # length ~ exponential-ish around avg_len; ensure at least 1
        length = max(1, int(random.expovariate(1.0 / avg_len)))
        r = min(max_coord, l + length)
        if r == l:
            r = min(max_coord, l + 1)
        out.append((l, r, None))
    return out

def gen_queries(nq: int, max_coord: int, width: int) -> List[Query]:
    qs: List[Query] = []
    for _ in range(nq):
        l = random.randrange(0, max(1, max_coord - max(1, width)))
        r = l + max(1, width)
        qs.append((l, r))
    return qs

# ---------------------- Adapters ----------------------
@dataclass
class Impl:
    name: str
    build: Callable[[List[Interval]], Any]          # returns a tree object
    query: Callable[[Any, int, int], int]           # returns number of hits (int) for minimal overhead

def _mk_intervaltree_py() -> Impl:

    def build(intervals: List[Interval]) -> IntervalTree:
        # IntervalTree accepts (begin, end, data); constructing in one shot is faster than incremental add
        return IntervalTree.from_tuples([(l, r, d) for (l, r, d) in intervals])

    def query(tree: Any, l: int, r: int) -> int:
        return len(tree.overlap(l, r))

    return Impl("intervaltree (PyPI)", build, query)

def _mk_intervaltree_rs() -> Impl:

    def build(intervals: List[Interval]) -> Any:
        # Try common APIs; adapt if yours differs
        tree = getattr(itrs, "IntervalTree")()
        add = getattr(tree, "add", None) or getattr(tree, "insert", None)
        if add is None:
            raise RuntimeError("intervaltree_rs: no add/insert method found")
        for (l, r, d) in intervals:
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
        # Avoid materialization cost if library returns an iterator-like structure
        try:
            return len(res)
        except TypeError:
            return sum(1 for _ in res)

    return Impl("intervaltree_rs", build, query)

ADAPTERS = []
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
    # You can still run to see single-impl curves, but comparison won’t render fairly.

# ---------------------- Timing helpers ----------------------
def time_best_of(repeats: int, fn: Callable[[], Any]) -> float:
    best = math.inf
    for _ in range(repeats):
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

    # Pre-generate queries for each width (same across impls for fairness)
    queries_by_w = {w: gen_queries(N_QUERIES, MAX_COORD, w) for w in QUERY_WIDTHS}

    for impl in ADAPTERS:
        # Build
        try:
            t_build = time_best_of(REPEATS, lambda: impl.build(intervals))
            # Rebuild once to get the tree we’ll reuse for search
            tree = impl.build(intervals)
        except MemoryError:
            t_build = math.nan
            tree = None
        build_times[impl.name].append(t_build)
        print(f"{impl.name}: build {t_build:.4f}s")

        if tree is None:
            for w in QUERY_WIDTHS:
                query_times[impl.name][w].append(math.nan)
                query_throughput[impl.name][w].append(math.nan)
            continue

        # Search (batch)
        for w, queries in queries_by_w.items():
            def run_queries() -> int:
                count = 0
                for (l, r) in queries:
                    count += impl.query(tree, l, r)
                return count

            t_query = time_best_of(REPEATS, run_queries)
            qps = N_QUERIES / t_query
            query_times[impl.name][w].append(t_query)
            query_throughput[impl.name][w].append(qps)
            print(f"{impl.name}: search width={w} total {t_query:.4f}s ({qps:.0f} q/s)")

# ---------------------- Plots ----------------------
# 1) Build time vs N
plt.figure()
for impl in ADAPTERS:
    plt.plot(SIZES, build_times[impl.name], marker="o", label=impl.name)
plt.xlabel("Number of intervals (N)")
plt.ylabel("Build time (s)")
plt.title("Build time vs N")
plt.legend()
plt.tight_layout()
plt.show()

# 2) Total search time vs N for each query width
for w in QUERY_WIDTHS:
    plt.figure()
    for impl in ADAPTERS:
        plt.plot(SIZES, query_times[impl.name][w], marker="o", label=impl.name)
    plt.xlabel("Number of intervals (N)")
    plt.ylabel(f"Total time for {N_QUERIES} queries (s)")
    plt.title(f"Search time vs N (query width={w})")
    plt.legend()
    plt.tight_layout()
    plt.show()

# 3) Throughput vs N (higher is better)
for w in QUERY_WIDTHS:
    plt.figure()
    for impl in ADAPTERS:
        plt.plot(SIZES, query_throughput[impl.name][w], marker="o", label=impl.name)
    plt.xlabel("Number of intervals (N)")
    plt.ylabel("Throughput (queries/sec)")
    plt.title(f"Search throughput vs N (query width={w})")
    plt.legend()
    plt.tight_layout()
    plt.show()
