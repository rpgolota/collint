from __future__ import annotations
from .collint import blackboard_rs, imitative_rs
from typing import Union

def blackboard(m: int, b: int, max_c: float = 10.0, compute_phi: bool = False) -> Union[dict, None]:
    
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
    
    result = blackboard_rs(m, b, max_c, compute_phi)
    if not result:
        return None
    m, b, t_star, phi, c = result
    ret = {
        "m": m,
        "b": b,
        "t_star": t_star,
        "c": c
    }
    if compute_phi:
        ret["phi"] = phi
    return ret

def imitative(m: int, p: float, max_c: float = 10.0) -> Union[dict, None]:
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
    
    result = imitative_rs(m, p, max_c)
    if not result:
        return None
    m, p, t_star, c = result
    ret = {
        "m": m,
        "p": p,
        "t_star": t_star,
        "c": c
    }
    return ret