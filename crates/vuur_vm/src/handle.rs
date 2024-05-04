use std::cell::RefCell;
pub use std::cell::{Ref, RefMut};
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
pub use std::rc::Weak;

/// Shared reference counted handle
pub struct Handle<T>(Rc<RefCell<T>>);

impl<T> Handle<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    #[inline(always)]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    #[inline(always)]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    #[inline(always)]
    pub fn try_borrow(&self) -> Option<Ref<'_, T>> {
        self.0.try_borrow().ok()
    }

    #[inline(always)]
    pub fn try_borrow_mut(&self) -> Option<RefMut<'_, T>> {
        self.0.try_borrow_mut().ok()
    }

    #[inline(always)]
    pub fn downgrade(&self) -> Weak<RefCell<T>> {
        Rc::downgrade(&self.0)
    }
}

impl<T> Clone for Handle<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> fmt::Debug for Handle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut debug = f.debug_tuple("Handle");

        match self.0.try_borrow() {
            Ok(value) => debug.field(&*value).finish(),
            Err(_) => debug.field(&"_").finish(),
        }
    }
}
