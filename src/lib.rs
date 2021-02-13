mod hack;
mod backend;
mod canvas;
mod chart;
mod range;
mod series;
mod dtype;
mod datetime;
mod mesh;

use backend::Backend;
use canvas::Canvas;
use chart::Chart;
use range::Range;
use series::Series;
use dtype::Dtype;
use datetime::DateTime;
use mesh::Mesh;

use pyo3::prelude::*;


#[pymodule]
fn ezel(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(pyo3::wrap_pyfunction!(count, m)?).unwrap();
    // m.add_class()

    // class list
    m.add_class::<Canvas>()?;
    m.add_class::<Chart>()?;
    m.add_class::<Range>()?;
    m.add_class::<DateTime>()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use pyo3::prelude::*;
    
    #[test]
    fn it_works() {
        pyo3::Python::with_gil(|py| {
            let list = pyo3::types::PyList::new(py, &[4, 1, 8, 5, 6, 2, 7, 3]);
            let canvas = Py::new(py, crate::Canvas::new(Some(400), Some(300))).unwrap();
            let chart = crate::Chart::new(canvas);
            // chart.line(pyo3::types::PyList::new(py, &[1,2,3]).extract::<Series>(), pyo3::types::PyList::new(py, &[3,2,1]).into());
        })
    }
}
