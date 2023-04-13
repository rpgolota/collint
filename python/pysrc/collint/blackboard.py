from .collint import blackboard_rs, blackboard_parallel_rs
from .util import (
    check_type_blackboard,
    check_type_blackboard_parallel,
    map_result_blackboard,
)
from .config import is_python_type_checking_enabled
from typing import Union


def blackboard(
    m: int, b: int, /, *, max_c: float = 10.0, compute_phi: bool = False
) -> Union[dict, None]:
    if is_python_type_checking_enabled():
        check_type_blackboard(m, b, max_c, compute_phi)
    return map_result_blackboard(blackboard_rs(m, b, max_c, compute_phi), compute_phi)


def blackboard_parallel(
    ms: "list[int]",
    bs: "list[int]",
    n: int,
    /,
    *,
    max_c: float = 10.0,
    compute_phi: bool = False,
    show_progress: bool = True,
) -> "list[Union[dict, None]]":
    if is_python_type_checking_enabled():
        check_type_blackboard_parallel(ms, bs, n, max_c, compute_phi, show_progress)
    results = blackboard_parallel_rs(ms, bs, n, max_c, compute_phi, show_progress)
    return [map_result_blackboard(r, compute_phi) for r in results]
