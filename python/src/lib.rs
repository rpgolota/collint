use collint_lib;
use pyo3::prelude::*;

#[pyfunction]
fn blackboard_rs(
    m: u32,
    b: u32,
    max_c: f64,
    compute_phi: bool,
) -> Option<(u32, u32, f64, f64, f64)> {
    collint_lib::blackboard(m, b, max_c, compute_phi).map_or(
        None,
        |collint_lib::BlackboardResult {
             m,
             b,
             t_star,
             phi,
             c,
         }| Some((m, b, t_star, phi, c)),
    )
}

#[pyfunction]
fn blackboard_parallel_rs(
    ms: Vec<u32>,
    bs: Vec<u32>,
    n: u32,
    max_c: f64,
    compute_phi: bool,
    show_progress: bool,
) -> Vec<Option<(u32, u32, f64, f64, f64)>> {
    collint_lib::blackboard_parallel(ms, bs, n, max_c, compute_phi, show_progress)
        .iter()
        .map(|r| match *r {
            Some(collint_lib::BlackboardResult {
                m,
                b,
                t_star,
                phi,
                c,
            }) => Some((m, b, t_star, phi, c)),
            None => None,
        })
        .collect()
}

#[pyfunction]
fn imitative_rs(m: u32, p: f64, max_c: f64) -> Option<(u32, f64, f64, f64)> {
    collint_lib::imitative(m, p, max_c)
        .map_or(None, |collint_lib::ImitativeResult { m, p, t_star, c }| {
            Some((m, p, t_star, c))
        })
}

#[pyfunction]
fn imitative_parallel_rs(
    ms: Vec<u32>,
    ps: Vec<f64>,
    n: u32,
    max_c: f64,
    show_progress: bool,
) -> Vec<Option<(u32, f64, f64, f64)>> {
    collint_lib::imitative_parallel(ms, ps, n, max_c, show_progress)
        .iter()
        .map(|r| match *r {
            Some(collint_lib::ImitativeResult { m, p, t_star, c }) => Some((m, p, t_star, c)),
            None => None,
        })
        .collect()
}

#[pymodule]
fn collint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(blackboard_rs, m)?)?;
    m.add_function(wrap_pyfunction!(blackboard_parallel_rs, m)?)?;
    m.add_function(wrap_pyfunction!(imitative_rs, m)?)?;
    m.add_function(wrap_pyfunction!(imitative_parallel_rs, m)?)?;
    Ok(())
}

