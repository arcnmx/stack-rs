use std::ops::{Deref, DerefMut};
use std::mem::{size_of, replace, forget};
use unreachable::{unreachable, UncheckedOptionExt};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum UnoptimizedOption<T> {
    Some(T),
    None,
}

impl<T> UnoptimizedOption<T> {
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            UnoptimizedOption::Some(ref v) => Some(v),
            UnoptimizedOption::None => None,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            UnoptimizedOption::Some(ref mut v) => Some(v),
            UnoptimizedOption::None => None,
        }
    }

    #[inline]
    pub fn take(&mut self) -> UnoptimizedOption<T> {
        replace(self, UnoptimizedOption::None)
    }
}

impl<T> UncheckedOptionExt<T> for UnoptimizedOption<T> {
    unsafe fn unchecked_unwrap(self) -> T {
        match self {
            UnoptimizedOption::Some(v) => v,
            UnoptimizedOption::None => unreachable(),
        }
    }

    unsafe fn unchecked_unwrap_none(self) {
        match self {
            UnoptimizedOption::Some(..) => unreachable(),
            UnoptimizedOption::None => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DropGuard<T>(UnoptimizedOption<T>);

impl<T> Drop for DropGuard<T> {
    fn drop(&mut self) {
        forget(self.0.take());
    }
}

impl<T> DropGuard<T> {
    pub fn new(t: T) -> Self {
        DropGuard(UnoptimizedOption::Some(t))
    }

    pub unsafe fn into_inner(&mut self) -> T {
        self.0.take().unchecked_unwrap()
    }
}

impl<T> Deref for DropGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref().unchecked_unwrap() }
    }
}

impl<T> DerefMut for DropGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut().unchecked_unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use super::DropGuard;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Dropper<'a>(&'a AtomicUsize);
    impl<'a> Drop for Dropper<'a> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn drop_guard() {
        let v = AtomicUsize::new(0);
        drop(Dropper(&v));
        drop(DropGuard::new(Dropper(&v)));
        assert_eq!(v.swap(0, Ordering::Relaxed), 1);
    }

    #[test]
    fn drop_guard_panic() {
        use std::sync::Arc;
        use std::thread::spawn;

        let v = Arc::new(AtomicUsize::new(0));
        let join = spawn({
            let v = v.clone();
            move || {
                let mut guard = DropGuard::new(Dropper(&v));
                let _guard_ref = &mut *guard;
                panic!();
            }
        });
        assert!(join.join().is_err());
        assert_eq!(v.swap(0, Ordering::Relaxed), 0);
    }
}
