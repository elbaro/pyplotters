use pyo3::{prelude::*, wrap_pymodule, wrap_pyfunction};

#[pyfunction]
fn doc(obj: PyObject) -> PyResult<()> {
    Ok(())
}

#[pymodule]
fn help_module(py:Python, m: &PyModule) -> PyResult<()> {
    // m.add_submodule(examples::example_module());
    m.add_function(pyo3::wrap_pyfunction!(doc, m)?).unwrap();
    // m.add_function(pyo3::wrap_pymodule!());
    Ok(())
}
