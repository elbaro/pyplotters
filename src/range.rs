use pyo3::prelude::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::Dtype;

/// Range represents a start and end.
/// It has no concept such as step or log scale.
#[pyclass]
#[derive(Clone, Copy)]
pub struct Range {
    pub range: RangeEnum
}

impl Range {
    pub fn dtype(&self) -> Dtype {
        match self.range {
            RangeEnum::I64(..) => Dtype::I64,
            RangeEnum::I32(..) => Dtype::I32,
            RangeEnum::F64(..) => Dtype::F64,
            RangeEnum::F32(..) => Dtype::F32,
            RangeEnum::DateTime(..) => Dtype::NaiveDateTime,
            RangeEnum::Date(..) => Dtype::NaiveDate,
            RangeEnum::Time(..) => Dtype::NaiveTime,
        }
    }
}

#[pymethods]
impl Range {
    #[staticmethod]
    pub fn f64(a: f64, b:f64) -> Self {Self {range: RangeEnum::F64(a,b)}}
    #[staticmethod]
    pub fn f32(a: f32, b:f32) -> Self {Self {range: RangeEnum::F32(a,b)}}
    #[staticmethod]
    pub fn i64(a: i64, b:i64) -> Self {Self {range: RangeEnum::I64(a,b)}}
    #[staticmethod]
    pub fn i32(a: i32, b:i32) -> Self {Self {range: RangeEnum::I32(a,b)}}
    #[staticmethod]
    pub fn date(a: pyo3_chrono::NaiveDate, b:pyo3_chrono::NaiveDate) -> Self {Self {range: RangeEnum::Date(a.0,b.0)}}
    #[staticmethod]
    pub fn datetime(a: pyo3_chrono::NaiveDateTime, b:pyo3_chrono::NaiveDateTime) -> Self {Self {range: RangeEnum::DateTime(a.0,b.0)}}
    #[staticmethod]
    pub fn time(a: pyo3_chrono::NaiveTime, b:pyo3_chrono::NaiveTime) -> Self {Self {range: RangeEnum::Time(a.0,b.0)}}
}

#[derive(Clone, Copy)]
pub enum RangeEnum {
    F64(f64,f64),
    F32(f32,f32),
    I64(i64,i64),
    I32(i32,i32),
    Date(NaiveDate,NaiveDate),
    DateTime(NaiveDateTime,NaiveDateTime),
    Time(NaiveTime,NaiveTime),
}
