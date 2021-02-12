//! Hacks to support unsafe operations.


/// cast (&'a T) to (&'static T).
/// This is used for self-referential struct.
/// TODO: use Pin and owning_ref crate
pub unsafe fn static_reference<'a, T>(x: &'a T) -> &'static T{
    std::mem::transmute::<_, _>(x)
}


// pub unsafe fn static_reference_mut<'a, T>(x: &'a mut T) -> &'static mut T{
//     std::mem::transmute::<_, _>(x)
// }

pub unsafe fn static_slice_mut<'a, T>(x: &'a mut [T]) -> &'static mut [T] {
    std::mem::transmute::<_, _>(x)
}
