use pyo3::{prelude::*, wrap_pymodule, wrap_pyfunction};
mod examples;

#[pyfunction]
fn doc(obj: &PyAny) -> PyResult<()> {
    todo!();
    // if Ok(x) = obj.extract::<>? {

    // }
    Ok(())
}

#[pymodule]
pub fn module(py:Python, _m: &PyModule) -> PyResult<()> {
    // m.add_submodule(examples::module);
    // m.add_function(pyo3::wrap_pyfunction!(doc, m)?).unwrap();
    // m.add_function(pyo3::wrap_pymodule!());
    Ok(())
}
