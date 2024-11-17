use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct SharedRef<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Default for SharedRef<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(T::default())),
        }
    }
}

impl<T> Clone for SharedRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct SharedRwRef<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Default for SharedRwRef<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(T::default())),
        }
    }
}

impl<T> Clone for SharedRwRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> SharedRwRef<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write().unwrap()
    }
}
