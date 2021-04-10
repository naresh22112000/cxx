use core::mem;
use core::ptr::{self, NonNull};
use core::slice;

#[repr(C)]
pub struct RustSlice {
    repr: NonNull<[()]>,
}

impl RustSlice {
    pub fn from_ref<T>(slice: &[T]) -> Self {
        let ptr = NonNull::from(slice).cast::<T>();
        let len = slice.len();
        Self::from_raw_parts(ptr, len)
    }

    pub fn from_mut<T>(slice: &mut [T]) -> Self {
        let ptr = NonNull::from(&mut *slice).cast::<T>();
        let len = slice.len();
        Self::from_raw_parts(ptr, len)
    }

    pub unsafe fn as_slice<'a, T>(self) -> &'a [T] {
        let ptr = self.as_ptr();
        let len = self.len();
        slice::from_raw_parts(ptr.as_ptr(), len)
    }

    pub unsafe fn as_mut_slice<'a, T>(self) -> &'a mut [T] {
        let ptr = self.as_ptr();
        let len = self.len();
        slice::from_raw_parts_mut(ptr.as_ptr(), len)
    }

    pub(crate) fn from_raw_parts<T>(ptr: NonNull<T>, len: usize) -> Self {
        // TODO: use NonNull::from_raw_parts(ptr.cast(), len) when stable.
        // https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html#method.from_raw_parts
        // https://github.com/rust-lang/rust/issues/81513
        let ptr = ptr::slice_from_raw_parts_mut(ptr.as_ptr().cast(), len);
        RustSlice {
            repr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }

    pub(crate) fn as_ptr<T>(&self) -> NonNull<T> {
        self.repr.cast()
    }

    pub(crate) fn len(&self) -> usize {
        // TODO: use self.repr.len() when stable.
        // https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html#method.len
        // https://github.com/rust-lang/rust/issues/71146
        let slice = unsafe { self.repr.as_ref() };
        slice.len()
    }
}

const_assert_eq!(
    mem::size_of::<Option<RustSlice>>(),
    mem::size_of::<RustSlice>(),
);
