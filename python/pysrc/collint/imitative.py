from .collint import imitative_rs, imitative_parallel_rs
from .util import (
    check_type_imitative,
    check_type_imitative_parallel,
    map_result_imitative,
)
from .config import is_python_type_checking_enabled
from typing import Union


def imitative(m: int, p: float, /, *, max_c: float = 10.0) -> Union[dict, None]:
    if is_python_type_checking_enabled():
        check_type_imitative(m, p, max_c)
    return map_result_imitative(imitative_rs(m, p, max_c))


def imitative_parallel(
    ms: "list[int]",
    ps: "list[float]",
    n: int,
    /,
    *,
    max_c: float = 10.0,
    show_progress: bool = True,
) -> "list[Union[dict, None]]":
    if is_python_type_checking_enabled():
        check_type_imitative_parallel(ms, ps, n, max_c, show_progress)
    results = imitative_parallel_rs(ms, ps, n, max_c, show_progress)
    return [map_result_imitative(r) for r in results]
