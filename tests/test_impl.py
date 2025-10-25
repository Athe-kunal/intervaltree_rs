import random
import pytest

itrs = pytest.importorskip("intervaltree_rs", reason="intervaltree_rs not installed")
intervaltree = pytest.importorskip(
    "intervaltree", reason="intervaltree (PyPI) not installed"
)


# ----------------- helpers -----------------
def gen_intervals(n: int, max_coord: int, avg_len: int, rng: random.Random):
    """Random intervals with positive length, clamped to [0, max_coord]."""
    out = []
    for i in range(n):
        l = rng.randrange(0, max_coord)
        length = max(1, int(rng.expovariate(1.0 / avg_len)))
        r = min(max_coord, l + length)
        if r == l:
            r = min(max_coord, l + 1)
        # carry some payload to ensure data is compared too
        out.append((l, r, f"id{i}"))
    return out


def gen_queries(nq: int, max_coord: int, width: int, rng: random.Random):
    """Fixed-width queries with ql <= qr, clamped to [0, max_coord]."""
    width = max(1, width)
    start_max = max(0, max_coord - width)
    qs = []
    for _ in range(nq):
        ql = rng.randrange(0, max(1, start_max))
        qr = min(max_coord, ql + width)
        qs.append((ql, qr))
    return qs


def build_rs(intervals):
    return itrs.IntervalTree.from_tuples(intervals)


def build_pypi(intervals):
    from intervaltree import IntervalTree

    return IntervalTree.from_tuples(intervals)

def hits_rs(tree_rs, ql: int, qr: int):
    # intervaltree_rs: inclusive [ql, qr]
    res = tree_rs.search(ql, qr, inclusive=False)
    # normalize to tuples
    return {(l, r, d) for (l, r, d) in res}


def hits_pypi(tree_py, ql: int, qr: int):
    res = tree_py[ql:qr]
    return {(iv.begin, iv.end, iv.data) for iv in res}


@pytest.mark.parametrize("seed", [7, 123, 99991])
@pytest.mark.parametrize("n", [500, 5000])  # tree sizes
@pytest.mark.parametrize("width", [1, 10, 1000, 10000])  # query widths
def test_rs_matches_pypi(seed, n, width):
    rng = random.Random(seed)
    MAX_COORD = 1_000_000
    AVG_LEN = 500
    NQ = 500  # number of queries to check

    intervals = gen_intervals(n, MAX_COORD, AVG_LEN, rng)
    qset = gen_queries(NQ, MAX_COORD, width, rng)

    tree_rs = build_rs(intervals)
    tree_py = build_pypi(intervals)

    for ql, qr in qset:
        rs_hits = sorted(hits_rs(tree_rs, ql, qr), key=lambda x: x[2])
        py_hits = sorted(hits_pypi(tree_py, ql, qr), key=lambda x: x[2])

        assert len(rs_hits) == len(py_hits), f"Mismatch for query [python = {len(py_hits)}, rust = {len(rs_hits)}] (NOT inclusive)"

        assert rs_hits == py_hits, f"Mismatch for query [{ql}, {qr}] (NOT inclusive)"
