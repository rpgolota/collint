def check_type_blackboard(m: int, b: int, max_c: float, compute_phi: bool):
    if type(m) is not int:
        raise ValueError("m must be an int.")
    if m <= 0:
        raise ValueError("m must be greater than 0.")
    if type(b) is not int:
        raise ValueError("b must be an int.")
    if b <= 0:
        raise ValueError("b must be greater than 0.")
    if type(max_c) is not float:
        raise ValueError("max_c must be a float.")
    if max_c < 0.0:
        raise ValueError("max_c must be greater than 0.")
    if type(compute_phi) is not bool:
        raise ValueError("compute_phi must be a bool.")


def check_type_blackboard_parallel(
    ms: "list[int]",
    bs: "list[int]",
    n: int,
    max_c: float,
    compute_phi: bool,
    show_progress: bool,
):
    if type(ms) is not list:
        raise ValueError("M must be a list.")
    if len(ms) == 0:
        raise ValueError("Must be at least one value in M.")
    if type(bs) is not list:
        raise ValueError("M must be a list.")
    if len(bs) == 0:
        raise ValueError("Must be at least one value in B.")

    for m in ms:
        if type(m) is not int:
            raise ValueError("values in M must be an int.")
        if m <= 0:
            raise ValueError("values in M must be greater than 0.")
    for b in bs:
        if type(b) is not int:
            raise ValueError("values in B must be an int.")
        if b <= 0:
            raise ValueError("values in B must be greater than 0.")

    if type(n) is not int:
        raise ValueError("n must be an int.")
    if n < 1:
        raise ValueError("n must be greater than 0.")

    if type(max_c) is not float:
        raise ValueError("max_c must be a float.")
    if max_c < 0.0:
        raise ValueError("max_c must be greater than 0.")
    if type(compute_phi) is not bool:
        raise ValueError("compute_phi must be a bool.")
    if type(show_progress) is not bool:
        raise ValueError("show_progress must be a bool.")


def check_type_imitative(m: int, p: float, max_c: float):
    if type(m) is not int:
        raise ValueError("m must be an int.")
    if m <= 0:
        raise ValueError("m must be greater than 0.")
    if type(p) is not float:
        raise ValueError("p must be a float.")
    if p < 0 or p > 1.0:
        raise ValueError("p must be between 0.0 and 1.0.")
    if type(max_c) is not float:
        raise ValueError("max_c must be a float.")
    if max_c < 0.0:
        raise ValueError("max_c must be greater than 0.")


def check_type_imitative_parallel(
    ms: "list[int]", ps: "list[float]", n: int, max_c: float, show_progress: bool
):
    if type(ms) is not list:
        raise ValueError("M must be a list.")
    if len(ms) == 0:
        raise ValueError("Must be at least one value in M.")
    if type(ps) is not list:
        raise ValueError("M must be a list.")
    if len(ps) == 0:
        raise ValueError("Must be at least one value in B.")

    for m in ms:
        if type(m) is not int:
            raise ValueError("values in M must be an int.")
        if m <= 0:
            raise ValueError("values in M must be greater than 0.")
    for p in ps:
        if type(p) is not float:
            raise ValueError("values in P must be a float.")
        if p < 0 or p > 1.0:
            raise ValueError("Values in P must be between 0.0 and 1.0.")

    if type(n) is not int:
        raise ValueError("n must be an int.")
    if n < 1:
        raise ValueError("n must be greater than 0.")

    if type(max_c) is not float:
        raise ValueError("max_c must be a float.")
    if max_c < 0.0:
        raise ValueError("max_c must be greater than 0.")
    if type(show_progress) is not bool:
        raise ValueError("show_progress must be a bool.")


def map_result_blackboard(result, compute_phi):
    if not result:
        return None

    m, b, t_star, phi, c = result
    ret = {"m": m, "b": b, "t_star": t_star, "c": c}
    if compute_phi:
        ret["phi"] = phi
    return ret


def map_result_imitative(result):
    if not result:
        return None

    m, p, t_star, c = result
    ret = {"m": m, "p": p, "t_star": t_star, "c": c}
    return ret
