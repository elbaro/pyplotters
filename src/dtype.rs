#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dtype {
    F64,
    F32,
    I64,
    I32,
    String,
    NaiveDateTime,
    NaiveDate,
    NaiveTime,
}

impl Dtype {
    pub fn is_numeric(&self) -> bool {
        match self {
            Dtype::F64 | Dtype::F32 | Dtype::I64 | Dtype::I32 => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Dtype::I64 | Dtype::I32 => true,
            _ => false,
        }
    }

    pub fn is_datetime(&self) -> bool {
        self == &Dtype::NaiveDateTime
    }

    pub fn is_chrono(&self) -> bool {
        match self {
            Dtype::NaiveDateTime | Dtype::NaiveDate | Dtype::NaiveTime => true,
            _ => false,
        }
    }
}
