use std::{
    cell::UnsafeCell,
    fmt::{self, Display},
    mem,
    ops::Deref,
};

pub struct SyncCell<T>(UnsafeCell<Option<T>>);

unsafe impl<T> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    pub fn init(&self, value: T) {
        debug_assert!(!self.initialized());
        unsafe {
            *self.0.get() = Some(value);
        }
    }

    pub fn get(&self) -> T {
        debug_assert!(self.initialized());
        unsafe { mem::take(&mut *self.0.get()).unwrap_unchecked() }
    }

    fn initialized(&self) -> bool {
        unsafe { (*self.0.get()).is_some() }
    }
}

impl<T> Default for SyncCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for SyncCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        debug_assert!(self.initialized());
        unsafe { (*self.0.get()).as_ref().unwrap_unchecked() }
    }
}

impl<T> Display for SyncCell<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
