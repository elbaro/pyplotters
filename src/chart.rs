use crate::hack::static_reference;
use crate::range::{Range, RangeEnum};
use crate::Canvas;
use crate::Dtype;
use crate::Series;
use plotters::coord::types::{RangedCoordf64, RangedDateTime};
use plotters::prelude::*;
use pyo3::prelude::*;

enum TypedChart {
    F64F64(
        ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ),
    DateTimeF64(
        ChartContext<
            'static,
            BitMapBackend<'static>,
            Cartesian2d<RangedDateTime<chrono::NaiveDateTime>, RangedCoordf64>,
        >,
    ),
    DateF64(
        ChartContext<
            'static,
            BitMapBackend<'static>,
            Cartesian2d<RangedDate<chrono::NaiveDate>, RangedCoordf64>,
        >,
    ),
    DurationF64(
        ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedDuration, RangedCoordf64>>,
    ),
}

#[pyclass(unsendable)]
#[text_signature = "(canvas, x_range, y_range, margin=None, margin_left=None ,margin_right=None, margin_top=None, margin_bottom=None, label_area=None, label_area_left=None, label_area_right=None, label_area_top=None, label_area_bottom=None, caption=None, caption_font=None, caption_size=None, mesh_x=None, mesh_y=None, axis_x=None, axis_y=None, axis_x_label_max=None, axis_y_label_max=None)"]
pub struct Chart {
    _canvas: Py<Canvas>, // Why Py<Canvas>? Since canvas is exposed to user, Python object around Canvas shouldn't be destroyed.
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
        _color: Option<&str>,
        // mesh (grid) and axis
        mesh_x: Option<bool>,
        mesh_y: Option<bool>,
        axis_x: Option<bool>,
        axis_y: Option<bool>,
        axis_x_label_max: Option<usize>,
        axis_y_label_max: Option<usize>,
    ) -> PyResult<Self> {
        let x_range = x_range.borrow(py);
        let y_range = y_range.borrow(py);
        let canvas_ref = canvas.borrow_mut(py);
        let mesh_x = mesh_x.unwrap_or(true);
        let mesh_y = mesh_y.unwrap_or(true);
        let axis_x = axis_x.unwrap_or(true);
        let axis_y = axis_y.unwrap_or(true);
        let axis_x_label_max = axis_x_label_max.unwrap_or(10);
        let axis_y_label_max = axis_y_label_max.unwrap_or(10);

        let mut b = ChartBuilder::on(unsafe { static_reference(&canvas_ref.area) });
        if let Some(v) = margin {
            b.margin(v);
        }
        if let Some(v) = margin_left {
            b.margin_left(v);
        }
        if let Some(v) = margin_right {
            b.margin_right(v);
        }
        if let Some(v) = margin_top {
            b.margin_top(v);
        }
        if let Some(v) = margin_bottom {
            b.margin_bottom(v);
        }

        // user specified no margin at all
        if margin.is_none()
            && margin_left.is_none()
            && margin_right.is_none()
            && margin_top.is_none()
            && margin_bottom.is_none()
        {
            b.margin_left(20); // TODO: change to percentage
            b.margin_right(20);
            b.margin_top(20);
            b.margin_bottom(20);
        }

        if label_area.is_none()
            && label_area_left.is_none()
            && label_area_right.is_none()
            && label_area_top.is_none()
            && label_area_bottom.is_none()
        {
            b.set_left_and_bottom_label_area_size(20);
        }

        if let Some(v) = label_area {
            b.set_all_label_area_size(v);
        }
        if let Some(v) = label_area_left {
            b.y_label_area_size(v);
        }
        if let Some(v) = label_area_right {
            b.right_y_label_area_size(v);
        }
        if let Some(v) = label_area_top {
            b.top_x_label_area_size(v);
        }
        if let Some(v) = label_area_bottom {
            b.x_label_area_size(v);
        }
        if let Some(v) = caption {
            b.caption(
                v,
                (
                    caption_font.unwrap_or("sans-serif"),
                    caption_size.unwrap_or(20),
                ),
            );
        }
        drop(canvas_ref);

        let x_dtype = x_range.dtype();
        let y_dtype = y_range.dtype();

        let inner = match (x_dtype, y_dtype) {
            (Dtype::F64, Dtype::F64) => {
                let x_range =
                    flowutils::unwrap_pattern!(x_range.range, RangeEnum::F64(a,b) => (a..b));
                let y_range =
                    flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                chart.plotting_area().fill(&WHITE).unwrap();
                {
                    let mut mesh = chart.configure_mesh();
                    if !mesh_x {
                        mesh.disable_x_mesh();
                    }
                    if !mesh_y {
                        mesh.disable_y_mesh();
                    }
                    if !axis_x {
                        mesh.disable_x_axis();
                    }
                    if !axis_y {
                        mesh.disable_y_axis();
                    }
                    mesh.x_labels(axis_x_label_max);
                    mesh.y_labels(axis_y_label_max);
                    mesh.draw().unwrap();
                }
                TypedChart::F64F64(chart)
            }
            (Dtype::NaiveDateTime, Dtype::F64) => {
                let x_range: RangedDateTime<chrono::NaiveDateTime> =
                    flowutils::unwrap_pattern!(x_range.range, RangeEnum::DateTime(a,b) => (a..b))
                        .into();
                let y_range =
                    flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                chart.plotting_area().fill(&WHITE).unwrap();
                {
                    let mut mesh = chart.configure_mesh();
                    if !mesh_x {
                        mesh.disable_x_mesh();
                    }
                    if !mesh_y {
                        mesh.disable_y_mesh();
                    }
                    if !axis_x {
                        mesh.disable_x_axis();
                    }
                    if !axis_y {
                        mesh.disable_y_axis();
                    }
                    mesh.x_labels(axis_x_label_max);
                    mesh.y_labels(axis_y_label_max);
                    mesh.draw().unwrap();
                }
                TypedChart::DateTimeF64(chart)
            }
            (Dtype::NaiveDate, Dtype::F64) => {
                let x_range: RangedDate<chrono::NaiveDate> =
                    flowutils::unwrap_pattern!(x_range.range, RangeEnum::Date(a,b) => (a..b))
                        .into();
                let y_range =
                    flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                chart.plotting_area().fill(&WHITE).unwrap();
                {
                    let mut mesh = chart.configure_mesh();
                    if !mesh_x {
                        mesh.disable_x_mesh();
                    }
                    if !mesh_y {
                        mesh.disable_y_mesh();
                    }
                    if !axis_x {
                        mesh.disable_x_axis();
                    }
                    if !axis_y {
                        mesh.disable_y_axis();
                    }
                    mesh.x_labels(axis_x_label_max);
                    mesh.y_labels(axis_y_label_max);
                    mesh.draw().unwrap();
                }
                TypedChart::DateF64(chart)
            }
            (Dtype::Duration, Dtype::F64) => {
                let x_range: RangedDuration =
                    flowutils::unwrap_pattern!(x_range.range, RangeEnum::Duration(a,b) => (a..b))
                        .into();
                let y_range =
                    flowutils::unwrap_pattern!(y_range.range, RangeEnum::F64(a,b) => (a..b));
                let mut chart = b.build_cartesian_2d(x_range, y_range).unwrap();
                chart.plotting_area().fill(&WHITE).unwrap();
                {
                    let mut mesh = chart.configure_mesh();
                    if !mesh_x {
                        mesh.disable_x_mesh();
                    }
                    if !mesh_y {
                        mesh.disable_y_mesh();
                    }
                    if !axis_x {
                        mesh.disable_x_axis();
                    }
                    if !axis_y {
                        mesh.disable_y_axis();
                    }
                    mesh.x_labels(axis_x_label_max);
                    mesh.y_labels(axis_y_label_max);
                    mesh.draw().unwrap();
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

    /// x and y accepts a native Python list or a 1D numpy.ndarray.
    /// _color parameter is not implemented yet.
    #[text_signature = "($self, x, y, _color=None, filled=None, stroke_width=None)"]
    pub fn line(
        &mut self,
        py: Python,
        x: Series,
        y: Series,
        _color: Option<&str>,
        filled: Option<bool>,
        stroke_width: Option<u32>,
    ) -> PyResult<()> {
        assert!(x.len(py) == y.len(py));
        let color = ShapeStyle {
            color: self.next_color().to_rgba(),
            filled: filled.unwrap_or(true),
            stroke_width: stroke_width.unwrap_or(3),
        };

        match &mut self.inner {
            TypedChart::F64F64(ref mut c) => {
                c.draw_series(LineSeries::new(x.iter_f64(py).zip(y.iter_f64(py)), color))
                    .unwrap();
            }
            TypedChart::DateTimeF64(ref mut c) => {
                c.draw_series(LineSeries::new(
                    x.iter_datetime(py).zip(y.iter_f64(py)),
                    color,
                ))
                .unwrap();
            }
            TypedChart::DateF64(ref mut c) => {
                c.draw_series(LineSeries::new(x.iter_date(py).zip(y.iter_f64(py)), color))
                    .unwrap();
            }
            TypedChart::DurationF64(ref mut c) => {
                c.draw_series(LineSeries::new(
                    x.iter_duration(py).zip(y.iter_f64(py)),
                    color,
                ))
                .unwrap();
            }
        }
        Ok(())
    }
    

    /// x and y accepts a native Python list or a 1D numpy.ndarray.
    /// _color parameter is not implemented yet.
    #[text_signature = "($self, x, y, size, _color=None, filled=None, stroke_width=None)"]
    pub fn scatter(
        &mut self,
        py: Python,
        x: Series,
        y: Series,
        size: Option<u32>,
        _color: Option<&str>,
        filled: Option<bool>,
        stroke_width: Option<u32>,
    ) -> PyResult<()> {
        assert!(x.len(py) == y.len(py));
        let size = size.unwrap_or(5);
        let color = ShapeStyle {
            color: self.next_color().to_rgba(),
            filled: filled.unwrap_or(true),
            stroke_width: stroke_width.unwrap_or(3),
        };

        match &mut self.inner {
            TypedChart::F64F64(ref mut c) => {
                c.draw_series(
                    x.iter_f64(py)
                        .zip(y.iter_f64(py))
                        .map(|(x, y)| Circle::new((x, y), size, color.clone())),
                )
                .unwrap();
            }
            TypedChart::DateTimeF64(ref mut c) => {
                c.draw_series(
                    x.iter_datetime(py)
                        .zip(y.iter_f64(py))
                        .map(|(x, y)| Circle::new((x, y), size, color.clone())),
                )
                .unwrap();
            }
            TypedChart::DateF64(ref mut c) => {
                c.draw_series(
                    x.iter_date(py)
                        .zip(y.iter_f64(py))
                        .map(|(x, y)| Circle::new((x, y), size, color.clone())),
                )
                .unwrap();
            }
            TypedChart::DurationF64(ref mut c) => {
                c.draw_series(
                    x.iter_duration(py)
                        .zip(y.iter_f64(py))
                        .map(|(x, y)| Circle::new((x, y), size, color.clone())),
                )
                .unwrap();
            }
        }

        Ok(())
    }
}
