//! Fixed size stack data structure.

use std::fmt::Debug;

/// Fixed size stack data structure backed by an array.
#[derive(Clone)]
pub struct ArrayStack<const N: usize, T> {
    data: [T; N],
    top: usize,
}

impl<const N: usize, T: Default + Copy> ArrayStack<N, T> {
    /// Create a new stack, and fill the backing array with default values.
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            top: 0,
        }
    }
}

impl<const N: usize, T: Default + Clone> ArrayStack<N, T> {
    pub fn pop(&mut self) -> T {
        match self.try_pop() {
            Ok(datum) => datum,
            Err(err) => {
                panic!("{err}")
            }
        }
    }

    pub fn try_pop(&mut self) -> Result<T, StackError> {
        match self.top.checked_sub(1) {
            Some(top) => {
                self.top = top;
                let datum = self.data[self.top].clone();
                self.data[self.top] = T::default();
                Ok(datum)
            }
            None => Err(StackError::Underflow),
        }
    }

    /// Remove `n` number of slots from the top of the stack.
    pub fn truncate(&mut self, n: usize) {
        let old_top = self.top;
        self.top = self.top.saturating_sub(n);
        // Set the elements to the default value.
        self.data[self.top..old_top].fill(T::default());
    }
}

impl<const N: usize, T> ArrayStack<N, T> {
    /// Returns the top value.
    pub fn top(&self) -> Option<&T> {
        self.top.checked_sub(1).and_then(|idx| self.data.get(idx))
    }

    /// Returns the top value.
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.top.checked_sub(1).and_then(|idx| self.data.get_mut(idx))
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.top
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn push(&mut self, value: T) {
        if let Err(err) = self.try_push(value) {
            panic!("{err}")
        }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), StackError> {
        if self.top < N - 1 {
            self.data[self.top] = value;
            self.top += 1;
            Ok(())
        } else {
            Err(StackError::Overflow)
        }
    }

    /// Retrieve the underlying array storage.
    pub fn raw(&self) -> &[T; N] {
        &self.data
    }
}

impl<const N: usize, T: std::fmt::Debug> std::fmt::Debug for ArrayStack<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.data[..self.top]).finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackError {
    Underflow,
    Overflow,
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Underflow => "stack array underflow",
            Self::Overflow => "stack array overflow",
        };

        std::fmt::Display::fmt(&message, f)
    }
}

impl std::error::Error for StackError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = ArrayStack::<8, u32>::new();
        assert_eq!(stack.top(), None);
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

        stack.push(3);
        assert_eq!(stack.top().copied(), Some(3));
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());

        stack.push(7);
        assert_eq!(stack.top().copied(), Some(7));

        stack.push(11);
        assert_eq!(stack.top().copied(), Some(11));

        println!("{stack:?}");
    }

    #[test]
    fn test_pop() {
        let mut stack = ArrayStack::<8, u32>::new();
        stack.push(3);
        stack.push(7);
        stack.push(11);
        println!("{stack:?}");

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), 11);
        assert_eq!(stack.pop(), 7);
        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());
        assert_eq!(stack.try_pop(), Err(StackError::Underflow));
    }

    #[test]
    fn test_truncate() {
        let mut stack = ArrayStack::<8, u32>::new();
        stack.push(3);
        stack.push(7);
        stack.push(11);
        println!("{stack:?}");

        stack.truncate(2);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.top().copied(), Some(3));

        // Ensure the truncated elements are set to zero.
        assert_eq!(stack.raw(), &[3, 0, 0, 0, 0, 0, 0, 0]);
    }
}
