use std::cmp::min;
use std::fmt::Debug;

// It's basically a queue with a maximum capacity that wraps around in
// the internal storage.
#[derive(Debug)]
pub struct RingVec<T: Debug> {
    inner: Vec<Option<T>>,
    capacity: usize, // in case we haven't allocated it all
    start: usize,
    len: usize, // number of items present
}

impl<T: Debug> RingVec<T> {

    /// Creates an empty RingVec
    pub fn new(capacity: usize) -> RingVec<T> {
        if capacity > 0 {
            RingVec { inner: Vec::new(), capacity, start: 0, len: 0 }
        } else {
            panic!("RingVec requires a nonzero capacity");
        }
    }

    /// Creates an empty RingVec with preallocated storage
    pub fn new_preallocated(capacity: usize) -> RingVec<T> {
        let inner = Vec::with_capacity(capacity);
        RingVec { inner, capacity, start: 0, len: 0 }
    }

    /// True if there is no spare capacity
    pub fn is_full(&self) -> bool { self.len == self.capacity }

    /// Returns the number of items in the RingVec
    pub fn len(&self) -> usize { self.inner.len() - self.start }

    /// Returns the maximum possible number of items.
    pub fn capacity(&self) -> usize { self.capacity }

    /// Returns a ref to the head of the queue, if any.
    pub fn peek(&self) -> Option<&T> {
        if self.len > 0 { self.inner[self.start].as_ref() }
        else { None }
    }

    /// Returns the next item in the queue, if any.
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let start = self.start;
            println!("before pop: {:?}", self);
            self.start = (self.start + 1) % self.capacity;
            self.len -= 1;
            let ret = self.inner[start].take();
            println!("after pop: {:?}", self);
            ret
        } else { None }
    }

    /// Appends an item if there is space.
    pub fn push(&mut self, item: T) -> Result<(), T> {
        println!("before push: {:?}", self);
        if self.len < self.capacity {
            let end = self.start + self.len;
            if end >= self.inner.capacity() {
                self.inner.push(Some(item));
            } else {
                self.inner[end % self.capacity] = Some(item);
            }
            self.len += 1;
            Ok(())
        } else {
            Err(item)
        }
    }

    /// Empties the RingVec, optionally shrinking the storage.
    pub fn empty(&mut self) {
        self.inner.clear();
        self.start = 0;
        self.len = 0;
    }

    /// Attempts to reduce memory usage.
    pub fn shrink_to_fit(&mut self) {
        if self.len == self.capacity { return; }
        let mut new = Vec::with_capacity(self.len());
        let end = self.start + self.len;
        for i in self.start..min(end, self.capacity) {
            new.push(self.inner[i].take());
        }
        if end > self.capacity {
            for i in 0..(end - self.capacity) {
                new.push(self.inner[i].take());
            }
        }
        self.inner = new;
    }

}

#[cfg(test)]
mod tests {

    use crate::RingVec;

    #[test]
    fn works() {
        let mut q: RingVec<usize> = RingVec::new(1);
        assert_eq!(q.peek(), None);
        assert_eq!(q.pop(), None);
        assert_eq!(q.push(1), Ok(()));
        assert_eq!(q.peek(), Some(&1));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.peek(), None);
        assert_eq!(q.pop(), None);
        assert_eq!(q.push(2), Ok(()));
        assert_eq!(q.peek(), Some(&2));
        assert_eq!(q.push(3), Err(3));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.push(4), Ok(()));
        assert_eq!(q.peek(), Some(&4));
        assert_eq!(q.pop(), Some(4));
        assert_eq!(q.peek(), None);
        assert_eq!(q.pop(), None);
        assert_eq!(q.peek(), None);
    }

}
