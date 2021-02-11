//! A type-erased wrapper around plotters::DrawingBackend and buffer.
//! This is not exposed to Python user and only used internally.
//!
//! For now, it only supports bitmap.

use plotters::prelude::*;
use crate::hack::static_slice_mut;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Backend {
    pub buffer: Vec<u8>,  // or String
    pub inner: Rc<RefCell<BitMapBackend<'static>>>  // TODO: generalize to SVG and wasm
}
impl Backend {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![0;width*height*3];
        let inner = Rc::new(RefCell::new(BitMapBackend::with_buffer(unsafe{static_slice_mut(&mut buffer)}, (width as u32, height as u32))));
        // let inner = Rc::new(RefCell::new(BitMapBackend::new("ezel.png", (width as u32, height as u32))));
        Self {
            buffer,
            inner,
        }
    }
}
