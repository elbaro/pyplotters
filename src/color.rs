// //! color1 = ezel.Color.turbo(0.3)
// //! color2 = ezel.Color.viridis(0.5)
// //! ..
// use pyo3::prelude::*;

// #[pyclass]
// struct ContinuousColorScheme {
//     inner: colorous::Gradient,
// }

// #[pymethods]
// impl ContinuousColorScheme {
//     pub fn any_scheme() -> Self {
//         let mut rng = rand::thread_rng();
//         match rng.gen::<usize>() % 10 {
//         }

//     }
//     pub fn pick(t: f64) -> colorous::Color {
//         Color{inner:self.inner.eval_continuous(t)}
//     }
// }

// #[pyclass]
// struct DiscreteColorScheme {
//     inner: &'static [colorous::Color; 10],
// }

// #[pymethods]
// impl DiscreteColorScheme {
//     pub fn pick(t: usize) -> colorous::Color {
//         Color{inner:self.inner[t]}
//     }
// }

// #[pyclass]
// struct Color {
//     inner: colorous::Color
// }

// macro_rules!();

// #[pymethods]
// impl Color {
//     fn turbo(t: f64) -> PyResult<> {
//         ;
//     }
// }
