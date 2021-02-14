//! A series is a wrapper around different types of series data.
//! It can be a Python list, numpy array, etc.
//!
//! Example:
//!
//! chart.line(
//!    x=range(5,7),              // this is Series::range
//!    y=np.array([1.0,2.0,3.0])  // this is Series::numpy
//! )
use std::collections::binary_heap::Iter;

use crate::Date as EzelDate;
use crate::DateTime as EzelDateTime;
use crate::Dtype;
use crate::Duration as EzelDuration;
use crate::Time as EzelTime;
use chrono::{NaiveDate, NaiveDateTime};
use numpy::array::PyArray1;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};

/// An adapter between Python series types (PyList, PyArray1, pandas.Series) and Rust.
/// This is used in the Python<->Rust API boundary to
///     1. avoid copy
///     2. easily cast to super type
pub enum Series {
    EmptyPyList,
    String(Py<PyList>),
    List { dtype: Dtype, list: Py<PyList> },
    NumpyF64(Py<PyArray1<f64>>),
    NumpyF32(Py<PyArray1<f32>>),
    NumpyI64(Py<PyArray1<i64>>),
    NumpyI32(Py<PyArray1<i32>>),
    EzelDateTime(Py<EzelDateTime>),
    EzelDate(Py<EzelDate>),
    EzelTime(Py<EzelTime>),
    EzelDuration(Py<EzelDuration>),
}

impl Series {
    pub fn dtype(&self) -> Dtype {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(x) => unreachable!(),
            Series::List { dtype, .. } => *dtype,
            Series::NumpyF64(..) => Dtype::F64,
            Series::NumpyF32(..) => Dtype::F32,
            Series::NumpyI64(..) => Dtype::I64,
            Series::NumpyI32(..) => Dtype::I32,
            Series::EzelDateTime(..) => Dtype::NaiveDateTime,
            Series::EzelDate(x) => Dtype::NaiveDate,
            Series::EzelTime(x) => Dtype::NaiveTime,
            Series::EzelDuration(x) => Dtype::Duration,
        }
    }
    pub fn len(&self, py: Python) -> usize {
        match self {
            Series::EmptyPyList => 0,
            Series::String(x) => x.as_ref(py).len(),
            Series::List { dtype: _, list: x } => x.as_ref(py).len(),
            Series::NumpyF64(x) => x.as_ref(py).len(),
            Series::NumpyF32(x) => x.as_ref(py).len(),
            Series::NumpyI64(x) => x.as_ref(py).len(),
            Series::NumpyI32(x) => x.as_ref(py).len(),
            Series::EzelDateTime(x) => x.borrow(py).len(),
            Series::EzelDate(x) => x.borrow(py).len(),
            Series::EzelTime(x) => x.borrow(py).len(),
            Series::EzelDuration(x) => x.borrow(py).len(),
        }
    }
    pub fn iter_f64<'a: 'out, 'py: 'out, 'out>(
        &'a self,
        py: Python<'py>,
    ) -> Box<dyn Iterator<Item = f64> + 'out> {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(..) => unreachable!(),
            Series::List {
                dtype: _, list: x, ..
            } => Box::new(
                x.as_ref(py)
                    .iter()
                    .map(|pyany| pyany.extract::<f64>().unwrap()),
            ),
            Series::NumpyF64(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| *x)),
            Series::NumpyF32(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| (*x) as f64)),
            Series::NumpyI64(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| (*x) as f64)),
            Series::NumpyI32(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| (*x) as f64)),
            Series::EzelDateTime(_) => unreachable!(),
            Series::EzelDate(..) => unreachable!(),
            Series::EzelTime(..) => unreachable!(),
            Series::EzelDuration(..) => unreachable!(),
        }
    }
    pub fn iter_i64<'a: 'out, 'py: 'out, 'out>(
        &'a self,
        py: Python<'py>,
    ) -> Box<dyn Iterator<Item = i64> + 'out> {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(..) => unreachable!(),
            Series::List {
                dtype: _, list: x, ..
            } => Box::new(
                x.as_ref(py)
                    .iter()
                    .map(|pyany| pyany.extract::<i64>().unwrap()),
            ),
            Series::NumpyF64(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| (*x) as i64)),
            Series::NumpyF32(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| (*x) as i64)),
            Series::NumpyI64(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| *x)),
            Series::NumpyI32(x) => Box::new(x.as_ref(py).iter().unwrap().map(|x| (*x) as i64)),
            Series::EzelDateTime(_) => unreachable!(),
            Series::EzelDate(..) => unreachable!(),
            Series::EzelTime(..) => unreachable!(),
            Series::EzelDuration(..) => unreachable!(),
        }
    }
    pub fn iter_datetime<'a: 'out, 'py: 'out, 'out>(
        &'a self,
        py: Python<'py>,
    ) -> Box<dyn Iterator<Item = chrono::NaiveDateTime> + 'out> {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(..) => unreachable!(),
            Series::List {
                dtype: Dtype::NaiveDateTime,
                list,
                ..
            } => Box::new(
                list.as_ref(py)
                    .iter()
                    .map(|pyany| pyany.extract::<pyo3_chrono::NaiveDateTime>().unwrap().0),
            ),
            Series::List { dtype: _, .. } => {
                unreachable!()
            }
            Series::NumpyF64(..) => unreachable!(),
            Series::NumpyF32(..) => unreachable!(),
            Series::NumpyI64(..) => unreachable!(),
            Series::NumpyI32(..) => unreachable!(),
            Series::EzelDateTime(dt) => {
                let dt = dt.borrow(py);
                Box::new(IterDateTime::new(dt))
            }
            Series::EzelDate(..) => unreachable!(),
            Series::EzelTime(..) => unreachable!(),
            Series::EzelDuration(..) => unreachable!(),
        }
    }
    pub fn iter_date<'a: 'out, 'py: 'out, 'out>(
        &'a self,
        py: Python<'py>,
    ) -> Box<dyn Iterator<Item = chrono::NaiveDate> + 'out> {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(..) => unreachable!(),
            Series::List {
                dtype: Dtype::NaiveDate,
                list,
                ..
            } => Box::new(
                list.as_ref(py)
                    .iter()
                    .map(|pyany| pyany.extract::<pyo3_chrono::NaiveDate>().unwrap().0),
            ),
            Series::List { dtype: _, .. } => {
                unreachable!()
            }
            Series::NumpyF64(..) => unreachable!(),
            Series::NumpyF32(..) => unreachable!(),
            Series::NumpyI64(..) => unreachable!(),
            Series::NumpyI32(..) => unreachable!(),
            Series::EzelDateTime(dt) => unreachable!(),
            Series::EzelDate(d) => {
                let d = d.borrow(py);
                Box::new(IterDate::new(d))
            }
            Series::EzelTime(..) => unreachable!(),
            Series::EzelDuration(..) => unreachable!(),
        }
    }
    pub fn iter_duration<'a: 'out, 'py: 'out, 'out>(
        &'a self,
        py: Python<'py>,
    ) -> Box<dyn Iterator<Item = chrono::Duration> + 'out> {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(..) => unreachable!(),
            Series::List {
                dtype: Dtype::Duration,
                list,
                ..
            } => Box::new(
                list.as_ref(py)
                    .iter()
                    .map(|pyany| pyany.extract::<pyo3_chrono::Duration>().unwrap().0),
            ),
            Series::List { dtype: _, .. } => {
                unreachable!()
            }
            Series::NumpyF64(..) => unreachable!(),
            Series::NumpyF32(..) => unreachable!(),
            Series::NumpyI64(..) => unreachable!(),
            Series::NumpyI32(..) => unreachable!(),
            Series::EzelDateTime(dt) => unreachable!(),
            Series::EzelDate(..) => unreachable!(),
            Series::EzelTime(..) => unreachable!(),
            Series::EzelDuration(d) => {
                let d = d.borrow(py);
                Box::new(IterDuration::new(d))
            }
            Series::EzelDuration(..) => unreachable!(),
        }
    }
    pub fn iter_str<'a: 'out, 'py: 'out, 'out>(
        &'a self,
        py: Python<'py>,
    ) -> Box<dyn Iterator<Item = &'out str> + 'out> {
        match self {
            Series::EmptyPyList => unreachable!(),
            Series::String(x) => Box::new(
                x.as_ref(py)
                    .iter()
                    .map(|pyany| pyany.extract::<&PyString>().unwrap().to_str().unwrap()),
            ),
            Series::List { .. } => unreachable!(),
            Series::NumpyF64(..) => unreachable!(),
            Series::NumpyF32(..) => unreachable!(),
            Series::NumpyI64(..) => unreachable!(),
            Series::NumpyI32(..) => unreachable!(),
            Series::EzelDateTime(..) => unreachable!(),
            Series::EzelDate(..) => unreachable!(),
            Series::EzelTime(..) => unreachable!(),
            Series::EzelDuration(..) => unreachable!(),
        }
    }
}

pub struct IterDateTime<'py> {
    dt: PyRef<'py, EzelDateTime>,
    idx: usize,
}

impl<'py> IterDateTime<'py> {
    pub fn new(dt: PyRef<'py, EzelDateTime>) -> Self {
        Self { dt, idx: 0 }
    }
}

impl<'py> Iterator for IterDateTime<'py> {
    type Item = chrono::NaiveDateTime;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        self.dt.vec.get(idx).map(|v| *v)
    }
}

pub struct IterDate<'py> {
    d: PyRef<'py, EzelDate>,
    idx: usize,
}

impl<'py> IterDate<'py> {
    pub fn new(d: PyRef<'py, EzelDate>) -> Self {
        Self { d, idx: 0 }
    }
}

impl<'py> Iterator for IterDate<'py> {
    type Item = chrono::NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        self.d.vec.get(idx).map(|v| *v)
    }
}

pub struct IterDuration<'py> {
    d: PyRef<'py, EzelDuration>,
    idx: usize,
}

impl<'py> IterDuration<'py> {
    pub fn new(d: PyRef<'py, EzelDuration>) -> Self {
        Self { d, idx: 0 }
    }
}

impl<'py> Iterator for IterDuration<'py> {
    type Item = chrono::Duration;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        self.d.vec.get(idx).map(|v| *v)
    }
}

impl<'source> FromPyObject<'source> for Series {
    fn extract(x: &'source PyAny) -> PyResult<Self> {
        if let Ok(arr) = x.extract::<&PyList>() {
            // infer dtype from PyList
            if arr.iter().all(|pyany| pyany.extract::<i64>().is_ok()) {
                return Ok(Series::List {
                    dtype: Dtype::I64,
                    list: arr.into(),
                });
            } else if arr.iter().all(|pyany| pyany.extract::<f64>().is_ok()) {
                return Ok(Series::List {
                    dtype: Dtype::F64,
                    list: arr.into(),
                });
            } else if arr
                .iter()
                .all(|pyany| pyany.extract::<pyo3_chrono::NaiveDateTime>().is_ok())
            {
                return Ok(Series::List {
                    dtype: Dtype::NaiveDateTime,
                    list: arr.into(),
                });
            } else if arr
                .iter()
                .all(|pyany| pyany.extract::<pyo3_chrono::NaiveDate>().is_ok())
            {
                return Ok(Series::List {
                    dtype: Dtype::NaiveDate,
                    list: arr.into(),
                });
            } else if arr
                .iter()
                .all(|pyany| pyany.extract::<pyo3_chrono::NaiveTime>().is_ok())
            {
                return Ok(Series::List {
                    dtype: Dtype::NaiveTime,
                    list: arr.into(),
                });
            } else if arr
                .iter()
                .all(|pyany| pyany.extract::<pyo3_chrono::Duration>().is_ok())
            {
                return Ok(Series::List {
                    dtype: Dtype::Duration,
                    list: arr.into(),
                });
            } else {
                todo!()
            }
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
        if let Ok(dt) = x.extract::<Py<EzelDateTime>>() {
            return Ok(Series::EzelDateTime(dt));
        }

        Err(pyo3::exceptions::PyValueError::new_err(
            "failed to convert sequence to series",
        ))
    }
}
