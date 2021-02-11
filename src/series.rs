//! A series is a wrapper around different types of series data.
//! It can be a Python list, numpy array, etc.
//!
//! Example:
//!
//! chart.line(
//!    x=range(5,7),              // this is Series::range
//!    y=np.array([1.0,2.0,3.0])  // this is Series::numpy
//! )

use pyo3::prelude::*;
use pyo3::types::PyList;
use numpy::array::PyArray1;

pub enum Series {
    List(Py<PyList>),
    NumpyF64(Py<PyArray1<f64>>),
    NumpyF32(Py<PyArray1<f32>>),
    NumpyI64(Py<PyArray1<i64>>),
    NumpyI32(Py<PyArray1<i32>>),
}

impl Series {
    // fn iter_f64(&self) {
    //     todo!();
    // }
}

impl<'source> FromPyObject<'source> for Series {
    fn extract(x: &'source PyAny) -> PyResult<Self> {
        if let Ok(arr) = x.extract::<&PyList>() {
            todo!();
            // return Ok(Series::List(arr.into_py(py)));
        }
        if let Ok(arr) = x.extract::<&PyArray1<f64>>() {
            return Ok(Series::NumpyF64(arr.to_owned()));
        }
        if let Ok(arr) = x.extract::<&PyArray1<f32>>() {
            return Ok(Series::NumpyF32(arr.to_owned()));
        }
        if let Ok(arr) = x.extract::<&PyArray1<i64>>() {
            return Ok(Series::NumpyI64(arr.to_owned()));
        }
        if let Ok(arr) = x.extract::<&PyArray1<i32>>() {
            return Ok(Series::NumpyI32(arr.to_owned()));
        }

        Err(pyo3::exceptions::PyValueError::new_err("failed to convert sequence to series"))
    }
}
