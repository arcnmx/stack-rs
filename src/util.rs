use std::mem::size_of;

pub trait PointerExt {
    fn uoffset(self, off: usize) -> Self;
}

impl<T> PointerExt for *const T {
    #[inline]
    fn uoffset(self, off: usize) -> Self {
        (self as usize + off * size_of::<T>()) as *const _
    }
}

impl<T> PointerExt for *mut T {
    #[inline]
    fn uoffset(self, off: usize) -> Self {
        (self as usize + off * size_of::<T>()) as *mut _
    }
}
