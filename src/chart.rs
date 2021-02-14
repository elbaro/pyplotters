use numpy::ShapeError;
use plotters::prelude::*;
use plotters::coord::types::{RangedCoordf64, RangedDateTime};
use pyo3::prelude::*;
use crate::Canvas;
use crate::Series;
use crate::Dtype;
use crate::range::{Range, RangeEnum};
use crate::hack::static_reference;


enum TypedChart {
    F64F64(ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedCoordf64, RangedCoordf64>>),
    DateTimeF64(ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedDateTime<chrono::NaiveDateTime>, RangedCoordf64>>),
    DateF64(ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedDate<chrono::NaiveDate>, RangedCoordf64>>),
    DurationF64(ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedDuration, RangedCoordf64>>),
}

#[pyclass(unsendable)]
pub struct Chart {
    _canvas: Py<Canvas>,  // Why Py<Canvas>? Since canvas is exposed to user, Python object around Canvas shouldn't be destroyed.
    inner: TypedChart,
    color_index: usize,
    x_dtype: Dtype,
    y_dtype: Dtype,
}

impl Chart {
    fn next_color(&mut self) -> PaletteColor<Palette9999> {
        self.color_index += 1;
        if PaletteColor::<Palette9999>::pick(self.color_index).rgb() == (255, 255, 255) {
            self.color_index += 1;
        }
        PaletteColor::<Palette9999>::pick(self.color_index)
    }
}

#[pymethods]
impl Chart {
    /// Creates new Chart on a Canvas.
    /// 
    /// ## Chart Styles
    /// caption: the title of the chart (TODO: caption font)
    ///     caption_size: caption font size in px
    ///     caption_font: "sans-serif", "serif", "monospace", or font name
    /// margin: the space between the canvas and the chart
    /// 
    /// ## Axis Styles
    /// x_label (TODO: font)
    /// y_label (TODO: font)
    /// label_area:            space(px) for the top/bottom/left/right label areas
    ///     label_area_x:      space(px) for top, bottom
    ///     label_area_y:      space(px) for left, right
    ///     label_area_left:   space(px) for the left label area. precedes label_area.
    ///     label_area_right:  space(px) for right left label area. precedes label_area.
    ///     label_area_top:    space(px) for the top label area. precedes label_area.
    ///     label_area_bottom: space(px) for the bottom label area. precedes label_area.
    ///
    #[new]
    pub fn new(
            py: Python,
            canvas: Py<Canvas>,
            x_range: Py<Range>,
            y_range: Py<Range>,
            //
            margin: Option<i32>,
            margin_left: Option<i32>,
            margin_right: Option<i32>,
            margin_top: Option<i32>,
            margin_bottom: Option<i32>,
            //
            label_area: Option<i32>,
            label_area_left: Option<i32>,
            label_area_right: Option<i32>,
            label_area_top: Option<i32>,
            label_area_bottom: Option<i32>,
            //
            caption: Option<&str>,
            caption_font: Option<&str>,
            caption_size: Option<u32>,
            //
            mesh: Option<bool>) -> PyResult<Self> {
        let x_range = x_range.borrow(py);
        let y_range = y_range.borrow(py);
        let canvas_ref = canvas.borrow_mut(py);
        let mesh = mesh.unwrap_or(true);
        let mut b = ChartBuilder::on(unsafe{static_reference(&canvas_ref.area)});
        if let Some(v) = margin { b.margin(v);  }        
        if let Some(v) = margin_left { b.margin_left(v);  }
        if let Some(v) = margin_right { b.margin_right(v);  }
        if let Some(v) = margin_top { b.margin_top(v);  }
        if let Some(v) = margin_bottom { b.margin_bottom(v);  }

        // user specified no margin at all
        if margin.is_none() && margin_left.is_none() && margin_right.is_none() && margin_top.is_none() && margin_bottom.is_none() {
            b.margin_left(20);  // TODO: change to percentage
            b.margin_right(20);
            b.margin_top(20);
            b.margin_bottom(20);
        }

        if label_area.is_none() && label_area_left.is_none() && label_area_right.is_none() && label_area_top.is_none() && label_area_bottom.is_none() {
            b.set_all_label_area_size(20);
        }

        if let Some(v) = label_area { b.set_all_label_area_size(v); }
        if let Some(v) = label_area_left { b.y_label_area_size(v); }
        if let Some(v) = label_area_right { b.right_y_label_area_size(v); }
        if let Some(v) = label_area_top { b.top_x_label_area_size(v); }
        if let Some(v) = label_area_bottom { b.x_label_area_size(v); }
        if let Some(v) = caption { b.caption(v, (caption_font.unwrap_or("sans-serif"), caption_size.unwrap_or(20))); }
        drop(canvas_ref);

        let x_dtype = x_range.dtype();
        let y_dtype = y_range.dtype();
        
        let inner = match (x_dtype, y_dtype) {
            (Dtype::F64, Dtype::F64) => {
                let x_range = flowutils::unwrap_pattern!(x_range.range, RangeEnum::F64(a,b) => (a..b));
                let y_range = flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                if mesh {
                    if label_area.is_none() {
                        b.set_all_label_area_size(100);
                    }
                }
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                if mesh {
                    chart.configure_mesh().draw().unwrap();
                }
                
                TypedChart::F64F64(chart)
            }
            (Dtype::NaiveDateTime, Dtype::F64) => {
                let x_range: RangedDateTime<chrono::NaiveDateTime> = flowutils::unwrap_pattern!(x_range.range, RangeEnum::DateTime(a,b) => (a..b)).into();
                let y_range = flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                if mesh {
                    if label_area.is_none() {
                        b.set_all_label_area_size(100);
                    }
                }
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                if mesh {
                    chart.configure_mesh().draw().unwrap();
                }
                TypedChart::DateTimeF64(chart)
            }
            (Dtype::NaiveDate, Dtype::F64) => {
                let x_range: RangedDate<chrono::NaiveDate> = flowutils::unwrap_pattern!(x_range.range, RangeEnum::Date(a,b) => (a..b)).into();
                let y_range = flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                if mesh {
                    if label_area.is_none() {
                        b.set_all_label_area_size(100);
                    }
                }
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                if mesh {
                    chart.configure_mesh().draw().unwrap();
                }
                TypedChart::DateF64(chart)
            }
            (Dtype::Duration, Dtype::F64) => {
                let x_range: RangedDuration = flowutils::unwrap_pattern!(x_range.range, RangeEnum::Duration(a,b) => (a..b)).into();
                let y_range = flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                if mesh {
                    if label_area.is_none() {
                        b.set_all_label_area_size(100);
                    }
                }
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                if mesh {
                    chart.configure_mesh().draw().unwrap();
                }
                TypedChart::DurationF64(chart)
            }
            _ => {
                println!("x {:?} y {:?}", x_dtype, y_dtype);
                unreachable!()
            }
        };
        Ok(Self {
            _canvas: canvas,
            inner,
            color_index: 0,
            x_dtype,
            y_dtype,
        })
    }
    
    pub fn line(&mut self, py: Python, x: Series, y: Series, _color: Option<&str>, filled: Option<bool>, stroke_width: Option<u32>) -> PyResult<()> {
        assert!(x.len(py) == y.len(py));
        let color = ShapeStyle {
            color: self.next_color().to_rgba(),
            filled: filled.unwrap_or(true),
            stroke_width: stroke_width.unwrap_or(10),
        };

        match &mut self.inner {
            TypedChart::F64F64(ref mut c) => {
                c.draw_series(LineSeries::new(x.iter_f64(py).zip(y.iter_f64(py)),color)).unwrap();
            }
            TypedChart::DateTimeF64(ref mut c) => {
                c.draw_series(LineSeries::new(x.iter_datetime(py).zip(y.iter_f64(py)),color)).unwrap();
            }
            TypedChart::DateF64(ref mut c) => {
                c.draw_series(LineSeries::new(x.iter_date(py).zip(y.iter_f64(py)),color)).unwrap();
            }
            TypedChart::DurationF64(ref mut c) => {
                c.draw_series(LineSeries::new(x.iter_duration(py).zip(y.iter_f64(py)),color)).unwrap();
            }
        }
        Ok(())
    }

    pub fn scatter(&mut self, py: Python, x: Series, y: Series, size: Option<u32>, _color: Option<&str>, filled: Option<bool>, stroke_width: Option<u32>) -> PyResult<()> {
        assert!(x.len(py) == y.len(py));
        let size = size.unwrap_or(3);
        let color = ShapeStyle {
            color: self.next_color().to_rgba(),
            filled: filled.unwrap_or(true),
            stroke_width: stroke_width.unwrap_or(10),
        };

        match &mut self.inner {
            TypedChart::F64F64(ref mut c) => {
                c.draw_series(x.iter_f64(py).zip(y.iter_f64(py)).map(|(x,y)| {
                    Circle::new((x,y),size,color.clone())
                })).unwrap();
            }
            TypedChart::DateTimeF64(ref mut c) => {
                c.draw_series(x.iter_datetime(py).zip(y.iter_f64(py)).map(|(x,y)| {
                    Circle::new((x,y),size,color.clone())
                })).unwrap();
            }
            _ => unreachable!()
        }

        Ok(())
    }
}
