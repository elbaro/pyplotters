#[derive(Clone, Copy)]
pub enum Dtype {
    F64,
    F32,
    I64,
    I32,
    String,
    NavieDateTime,
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
}
