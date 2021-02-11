use plotters::prelude::*;
use plotters::coord::types::{RangedCoordf64};
use pyo3::prelude::*;
use crate::canvas::Canvas;
use crate::series::Series;
use crate::range::{Range, RangeEnum};
use crate::hack::static_reference;
use pyo3::types::PyList;
use numpy::array::PyArray1;


#[pyclass(unsendable)]
pub struct Chart {
    canvas: Py<Canvas>,  // Why Py<Canvas>? Since canvas is exposed to user, Python object around Canvas shouldn't be destroyed.
    inner: ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
}

#[pymethods]
impl Chart {
    #[new]
    pub fn new(py: Python, py_canvas: Py<Canvas>, x_range: Py<Range>, y_range: Py<Range>, margin: Option<i32>, label_area: Option<i32>, caption: Option<String>) -> PyResult<Self> {
        let x_range = x_range.borrow(py);
        let y_range = y_range.borrow(py);
        let canvas = py_canvas.borrow_mut(py);
        let mut b = ChartBuilder::on(unsafe{static_reference(&canvas.area)});
        if let Some(v) = margin { b.margin(v);  }
        if let Some(v) = label_area { b.set_all_label_area_size(v); }
        if let Some(v) = caption { b.caption(v, ("sans-serif", 20)); }
        drop(canvas);

        let x_range = flowutils::unwrap_pattern!(x_range.range, RangeEnum::F64(a,b) => (a..b));
        let y_range = flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
        Ok(Self {
            canvas: py_canvas,
            inner: b.build_cartesian_2d(x_range, y_range).unwrap(),
        })
    }

    
    /// 
    pub fn line(&mut self, py: Python, x: Series, y: Series) -> PyResult<()> {
        // let x: Py<PyList> = flowutils::unwrap_pattern!(x, Series::List(list) => list);
        // let x = x.as_ref(py);
        // let x = x.iter().map(|pyany| pyany.extract::<f64>().unwrap());
        // let y: Py<PyList> = flowutils::unwrap_pattern!(y, Series::List(list) => list);
        // let y = y.as_ref(py);
        // let y = y.iter().map(|pyany| pyany.extract::<f64>().unwrap());

        let x: Py<PyArray1<f64>> = flowutils::unwrap_pattern!(x, Series::NumpyF64(arr) => arr);
        let x = x.as_ref(py);
        let x = x.iter().unwrap().map(|x| *x);
        let y: Py<PyArray1<f64>> = flowutils::unwrap_pattern!(y, Series::NumpyF64(arr) => arr);
        let y = y.as_ref(py);
        let y = y.iter().unwrap().map(|y| *y);

        self.inner.draw_series(LineSeries::new(x.zip(y),&GREEN)).unwrap();
        Ok(())
    }
}
