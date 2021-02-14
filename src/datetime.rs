//! Because there are so many possible chrono type conversions,
//! we wrap chrono data in ezel::{DateTime, Date, Time} before wrapping with ezel::Series.
//!
//! For example, ezel::Date can be constructed from Python's
//! - [datetime::datetime]
//! - [datetime::date]
//! - [int] ( s timestamp)
//! - [int] (ms timestamp)
//! - [int] (us timestamp)
//! - [int] (ns timestamp)
//! - [str] (iso8601)
//! - [str] (custom format)

//! ezel.DateTime.timestamp_ns(arr)
//! ezel.DateTime.timestamp_us(arr)
//! ezel.DateTime.timestamp_ms(arr)
//! ezel.DateTime.timestamp_sec(arr)
//! ezel.DateTime.iso8601(arr)
//! ezel.DateTime.parse(arr, "%Y %b %d %H %M %S")

use crate::Series;
use pyo3::prelude::*;
use std::str::FromStr;

#[pyclass]
pub struct DateTime {
    pub vec: Vec<chrono::NaiveDateTime>,
}

impl DateTime {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[pymethods]
impl DateTime {
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_ns(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| {
                    chrono::NaiveDateTime::from_timestamp(
                        x / 1_000_000_000,
                        (x % 1_000_000_000) as u32,
                    )
                })
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_us(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| {
                    chrono::NaiveDateTime::from_timestamp(x / 1_000_000, (x % 1_000_000) as u32)
                })
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_ms(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x / 1_000, (x % 1_000) as u32))
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_sec(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x, 0))
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn iso8601(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_string());
        Self {
            vec: series
                .iter_str(py)
                .map(|x| chrono::NaiveDateTime::from_str(x).unwrap())
                .collect(),
        }
    }

    /// The format-string syntax follows Rust's format::strftime fuction.
    #[staticmethod]
    #[text_signature = "(series, fmt)"]
    pub fn parse(py: Python, series: Series, fmt: &str) -> Self {
        assert!(series.dtype().is_string());
        Self {
            vec: series
                .iter_str(py)
                .map(|x| chrono::NaiveDateTime::parse_from_str(x, fmt).unwrap())
                .collect(),
        }
    }
}

#[pyclass]
pub struct Date {
    pub vec: Vec<chrono::NaiveDate>,
}

#[pymethods]
impl Date {
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_ns(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x / 1_000_000_000, 0).date())
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_us(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x / 1_000_000, 0).date())
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_ms(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x / 1_000, 0).date())
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_sec(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x, 0).date())
                .collect(),
        }
    }
    // #[staticmethod]
    // pub fn iso8601(series: Series) -> Self {
    //     todo!();
    // }
    // #[staticmethod]
    // pub fn parse() -> Self {
    //     todo!();
    // }
}

impl Date {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[pyclass]
pub struct Time {
    vec: Vec<chrono::NaiveTime>,
}

#[pymethods]
impl Time {
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_ns(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| {
                    chrono::NaiveDateTime::from_timestamp(
                        x / 1_000_000_000,
                        (x % 1_000_000_000) as u32,
                    )
                    .time()
                })
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_us(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| {
                    chrono::NaiveDateTime::from_timestamp(x / 1_000_000, (x % 1_000_000) as u32)
                        .time()
                })
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_ms(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| {
                    chrono::NaiveDateTime::from_timestamp(x / 1_000, (x % 1_000) as u32).time()
                })
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn timestamp_sec(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::NaiveDateTime::from_timestamp(x, 0).time())
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn iso8601(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_string());
        Self {
            vec: series
                .iter_str(py)
                .map(|x| chrono::NaiveDateTime::from_str(x).unwrap().time())
                .collect(),
        }
    }

    /// The format-string syntax follows Rust's format::strftime fuction.
    #[staticmethod]
    #[text_signature = "(series, fmt)"]
    pub fn parse(py: Python, series: Series, fmt: &str) -> Self {
        assert!(series.dtype().is_string());
        Self {
            vec: series
                .iter_str(py)
                .map(|x| {
                    chrono::NaiveDateTime::parse_from_str(x, fmt)
                        .unwrap()
                        .time()
                })
                .collect(),
        }
    }
}

impl Time {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[pyclass]
pub struct Duration {
    pub vec: Vec<chrono::Duration>,
}

#[pymethods]
impl Duration {
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn ns(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::Duration::nanoseconds(x))
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn us(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::Duration::microseconds(x))
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn ms(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::Duration::milliseconds(x))
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn sec(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_integer());
        Self {
            vec: series
                .iter_i64(py)
                .map(|x| chrono::Duration::seconds(x))
                .collect(),
        }
    }
    #[staticmethod]
    #[text_signature = "(series)"]
    pub fn iso8601(py: Python, series: Series) -> Self {
        assert!(series.dtype().is_string());
        todo!();
    }

    /// The format-string syntax follows Rust's format::strftime fuction.
    #[staticmethod]
    #[text_signature = "(series, fmt)"]
    pub fn parse(py: Python, series: Series, fmt: &str) -> Self {
        assert!(series.dtype().is_string());
        todo!();
    }
}

impl Duration {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}
