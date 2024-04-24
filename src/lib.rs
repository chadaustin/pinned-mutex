use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard};

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
        let guard = self.get_ref()
            .inner
            .lock()
            .expect("PinnedMutex does not expose poison");
        PinnedMutexGuard { guard }
    }
}

pub struct PinnedMutexGuard<'a, T: 'a> {
    guard: MutexGuard<'a, T>,
}

impl<'a, T> Deref for PinnedMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T: Unpin> DerefMut for PinnedMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

// TODO: impl Drop

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::pin;

    #[test]
    fn it_works() {
        let pm = pin!(PinnedMutex::new(15));
        let mut locked = pm.as_ref().lock();
        *locked = 16;
    }
}
