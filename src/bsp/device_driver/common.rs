//! Common device driver code.

use core::{marker::PhantomData, ops};


pub struct MMIODerefWrapper<T> {
    base_addr: usize,
    phantom: PhantomData<T>,
}

impl<T> MMIODerefWrapper<T> {
    /// Create an instance
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            phantom: PhantomData,
        }
    }

    /// Return a pointer to the associated MMIO register block.
    fn ptr(&self) -> *const T {
        self.base_addr as *const _
    }
}

impl ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

