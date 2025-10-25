from typing import Any, Iterable, List, Tuple, Tuple, Generic, TypeAlias, TypeVar, overload

T = TypeVar("T")
StartInt: TypeAlias = int
EndInt: TypeAlias = int
Interval: TypeAlias = Tuple[StartInt,EndInt,T]
Key = Tuple[StartInt, EndInt]
Hit = Tuple[StartInt,EndInt,T]

class IntervalTree(Generic[T]):
    def __init__(self) -> None: ...

    @classmethod
    def from_tuples(cls, intervals: Iterable[IntervalTree[T]]) -> "IntervalTree": ...
    def insert(self, interval: Interval[T]) -> None: ...
    def delete(self, key: Key) -> bool: ...


    @overload
    def search(self, ql: int, qr: int) -> List[Hit[T]]:...
    @overload
    def search(self, ql: int, qr: int, inclusive: bool = True) -> List[Hit[T]]:...

    def is_empty(self) -> bool: ...
