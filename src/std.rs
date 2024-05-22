use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard};

/// Provides [structural
/// pinning](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning)
/// atop [Mutex].
pub struct PinnedMutex<T> {
    inner: Mutex<T>,
}

impl<T> PinnedMutex<T> {
    pub fn new(init: T) -> Self {
        Self {
            inner: Mutex::new(init),
        }
    }

    /// Acquires the lock and returns a guard.
    ///
    /// Poisoning is not supported. If the underlying mutex is
    /// poisoned, `lock` will panic.
    pub fn lock(self: Pin<&Self>) -> PinnedMutexGuard<'_, T> {
        let guard = self
            .get_ref()
            .inner
            .lock()
            .expect("PinnedMutex does not expose poison");
        PinnedMutexGuard { guard }
    }
}

/// Provides access to mutex's contents. [Deref] to `&T` is always
/// possible. [DerefMut] to `&mut T` is only possive if T is `Unpin`.
///
/// `as_ref` and `as_mut` project structural pinning.
pub struct PinnedMutexGuard<'a, T: 'a> {
    guard: MutexGuard<'a, T>,
}

impl<'a, T> PinnedMutexGuard<'a, T> {
    /// Provides pinned access to the underlying T.
    pub fn as_ref(&self) -> Pin<&T> {
        // PinnedMutex::lock requires the mutex is pinned.
        unsafe { Pin::new_unchecked(&self.guard) }
    }

    /// Provides pinned mutable access to the underlying T.
    pub fn as_mut(&mut self) -> Pin<&mut T> {
        // PinnedMutex::lock requires the mutex is pinned.
        // &mut self guarantees as_ref() cannot alias.
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
        // SAFETY: T is Unpin, so it's safe to move out of T.
        &mut self.guard
    }
}

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

    #[test]
    fn ref_alias() {
        let pm = pin!(PinnedMutex::new(MustPin::new()));
        let locked = pm.as_ref().lock();
        let a = locked.as_ref();
        let b = locked.as_ref();
        assert_eq!(a.value, b.value);
    }
}
