use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult};
pub mod run_java;

#[pymodule]
fn pyo3_j4rs_example(_: Python, m: &PyModule) -> PyResult<()> {

    m.add_wrapped(wrap_pyfunction!(my_func))?;
    Ok(())
}

#[pyfunction]
pub fn my_func<'a>(
    in_str: &str,
) -> PyResult<()> {
    run_java::run_java(in_str);
    Ok(())
}
