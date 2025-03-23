use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// A simple function to add two numbers
#[pyfunction]
fn add_two(a: i32) -> i32 {
    a + 2
}

// The module that will be imported in Python
#[pymodule]
fn engine_backend(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_two, m)?)?;
    Ok(())
}
