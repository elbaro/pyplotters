use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Mesh {
}

#[pymethods]
impl Mesh {
    #[new]
    pub fn new() -> Self {
        Self {
        }
    }
}

