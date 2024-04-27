use parking_lot::{Mutex, MutexGuard};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

pub struct PinnedMutex<T> {
    inner: Mutex<T>,
}

impl<T> PinnedMutex<T> {
    pub fn new(init: T) -> Self {
        Self {
            inner: Mutex::new(init),
        }
    }

    pub fn lock(self: Pin<&Self>) -> PinnedMutexGuard<'_, T> {
        let guard = self.get_ref().inner.lock();
        PinnedMutexGuard { guard }
    }
}

pub struct PinnedMutexGuard<'a, T: 'a> {
    guard: MutexGuard<'a, T>,
}

impl<'a, T> PinnedMutexGuard<'a, T> {
    pub fn as_ref(&mut self) -> Pin<&T> {
        unsafe { Pin::new_unchecked(&self.guard) }
    }

    pub fn as_mut(&mut self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut self.guard) }
    }
}

impl<'a, T> Deref for PinnedMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T: Unpin> DerefMut for PinnedMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

// TODO: impl Drop

#[cfg(test)]
mod tests {
    use super::*;
    use pin_project::pin_project;
    use std::{marker::PhantomPinned, pin::pin};

    #[test]
    fn mutate_through_lock() {
        let pm = pin!(PinnedMutex::new(15));
        let mut locked = pm.as_ref().lock();
        *locked = 16;
    }

    #[pin_project(UnsafeUnpin)]
    struct MustPin {
        value: u32,
        pinned: PhantomPinned,
    }

    impl MustPin {
        fn new() -> Self {
            Self {
                value: 0,
                pinned: PhantomPinned,
            }
        }

        fn inc(self: Pin<&mut Self>) -> u32 {
            let value = self.project().value;
            let prev = *value;
            *value += 1;
            prev
        }

        fn get(self: Pin<&Self>) -> u32 {
            *self.project_ref().value
        }
    }

    #[test]
    fn pinned_method() {
        let pm = pin!(PinnedMutex::new(MustPin::new()));
        let mut locked = pm.as_ref().lock();
        assert_eq!(0, locked.as_mut().inc());
        assert_eq!(1, locked.as_mut().inc());
        assert_eq!(2, locked.as_ref().get());
    }
}
