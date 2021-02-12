use pyo3::prelude::*;
use crate::Series;

#[pyclass]
pub struct DateTime {
    pub vec: Vec<chrono::NaiveDateTime>,
}

impl DateTime {
    pub fn len(&self)->usize {
        self.vec.len()
    }
}

// ezel.DateTime.timestamp_ns(arr)
// ezel.DateTime.timestamp_us(arr)
// ezel.DateTime.timestamp_ms(arr)
// ezel.DateTime.timestamp_sec(arr)
// ezel.DateTime.iso8601(arr)
// ezel.DateTime.parse(arr, "%Y %b %d %H %M %S")


#[pymethods]
impl DateTime {
    #[staticmethod]
    pub fn timestamp_ns(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self { vec: series.iter_i64(py).map(|x| chrono::NaiveDateTime::from_timestamp(x/1_000_000_000, (x%1_000_000_000) as u32)).collect() }
    }
    #[staticmethod]
    pub fn timestamp_us(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self { vec: series.iter_i64(py).map(|x| chrono::NaiveDateTime::from_timestamp(x/1_000_000, (x%1_000_000) as u32)).collect() }
    }
    #[staticmethod]
    pub fn timestamp_ms(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self { vec: series.iter_i64(py).map(|x| chrono::NaiveDateTime::from_timestamp(x/1_000, (x%1_000) as u32)).collect() }
    }
    #[staticmethod]
    pub fn timestamp_sec(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self { vec: series.iter_i64(py).map(|x| chrono::NaiveDateTime::from_timestamp(x, 0)).collect() }
    }
    #[staticmethod]
    pub fn iso8601(series: Series) {
    }
    #[staticmethod]
    pub fn parse() {
    }
}
