use collint_rs;
use pyo3::prelude::*;

#[pyfunction]
fn blackboard_rs(
    m: u32,
    b: u32,
    max_c: f64,
    compute_phi: bool,
) -> Option<(u32, u32, f64, f64, f64)> {
    collint_rs::blackboard(m, b, max_c, compute_phi).map_or(
        None,
        |collint_rs::BlackboardResult {
             m,
             b,
             t_star,
             phi,
             c,
         }| Some((m, b, t_star, phi, c)),
    )
}

#[pyfunction]
fn imitative_rs(m: u32, p: f64, max_c: f64) -> Option<(u32, f64, f64, f64)> {
    collint_rs::imitative(m, p, max_c)
        .map_or(None, |collint_rs::ImitativeResult { m, p, t_star, c }| {
            Some((m, p, t_star, c))
        })
}

#[pymodule]
fn collint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(blackboard_rs, m)?)?;
    m.add_function(wrap_pyfunction!(imitative_rs, m)?)?;
    Ok(())
}
