use std::sync::{Arc, Mutex, RwLock};

pub struct SharedRwRef<T> {
    inner: Arc<RwLock<T>>,
}

#[allow(dead_code)]
impl<T> Clone for SharedRwRef<T> {
    fn clone(&self) -> Self {
        SharedRwRef {
            inner: self.inner.clone(),
        }
    }
}

#[allow(dead_code)]
impl<T: Default> Default for SharedRwRef<T> {
    fn default() -> Self {
        SharedRwRef {
            inner: Arc::new(RwLock::new(T::default())),
        }
    }
}

#[allow(dead_code)]
impl<T> SharedRwRef<T> {
    pub fn new(value: T) -> Self {
        SharedRwRef {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    pub fn read(&self) -> std::sync::RwLockReadGuard<T> {
        self.inner.read().unwrap()
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<T> {
        self.inner.write().unwrap()
    }
}

pub struct SharedRef<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for SharedRef<T> {
    fn clone(&self) -> Self {
        SharedRef {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Default> Default for SharedRef<T> {
    fn default() -> Self {
        SharedRef {
            inner: Arc::new(Mutex::new(T::default())),
        }
    }
}
