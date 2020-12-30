pub struct RingBuffer<T> {
    data: Vec<T>,
    front: usize, // First to remove
    end: usize, // Last to remove
    capacity: usize, // Fixed
    is_empty: bool, // To distinguish 1 element and none
}

impl<T> RingBuffer<T>
where T: Copy + Default
{
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![T::default(); capacity],
            front: 0, // Pop from front
            end: 0, // Push to end
            capacity,
            is_empty: true,
        }
    }

    /// Create a new RingBuffer from an existing slice, in ascending order of indices in the slice
    pub fn new_from_slice(source: &[T]) -> Self {
        let mut buf = Self::new(source.len());
        for i in source.iter() {
            buf.add(*i);
        }
        buf
    }

    pub fn size(&self) -> usize {
        if self.end > self.front {
            self.end - self.front + 1
        } else if self.end < self.front {
            self.capacity - self.front + self.end + 1
        } else if self.is_empty {
            0
        } else {
            1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn add(&mut self, item: T) {
        if self.is_empty {
            self.is_empty = false;
            // Reset to play safe
            self.front = 0;
            self.end = 0;
        } else {
            self.increment_end();
            if self.front == self.end {
                self.increment_front();
            }
        }
        self.data[self.end] = item;
    }

    /// Add items to the RingBuffer, in ascending order of indices in the slice
    pub fn add_from_slice(&mut self, items: &[T]) {
        for item in items.iter() {
            self.add(*item);
        }
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty {
            None
        } else {
            Some(self.data[self.front])
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.is_empty {
            None
        } else {
            let temp: T = self.data[self.front];
            if self.front == self.end { // Removing the only one
                self.is_empty = true;
            } else {
                self.increment_front();
            }
            Some(temp)
        }
    }

    fn increment_front(&mut self) {
        self.front += 1;
        self.front %= self.capacity;
    }

    fn increment_end(&mut self) {
        self.end += 1;
        self.end %= self.capacity;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_ring_buffer() {
        let buf = RingBuffer::<i32>::new(7);
        assert!(buf.is_empty);
    }

    #[test]
    fn add_to_ring_buffer() {
        let mut buf = RingBuffer::<i32>::new(7);

        buf.add(3);
        if let Some(val) = buf.peek() {
            assert!(buf.size() == 1);
            assert!(val == 3);
        } else {
            panic!();
        }

        buf.add(4);
        if let Some(val) = buf.peek() {
            assert!(val == 3); // still 3 because it is a queue
        } else {
            panic!();
        }
        
        assert!(buf.size() == 2);

        if let Some(val) = buf.remove() {
            assert!(val == 3);
        } else {
            panic!();
        }

        assert!(buf.size() == 1);

        if let Some(val) = buf.peek() {
            assert!(!buf.is_empty());
            assert!(val == 4); // 4 because 3 is dequeued
        } else {
            panic!();
        }

        assert!(buf.size() == 1);
    }

    #[test]
    fn remove_from_ring_buffer() {
        let mut buf = RingBuffer::<i32>::new(7);

        if buf.remove().is_some() {
            panic!()
        }

        for i in 1..=7 {
            buf.add(i);
            assert!(buf.size() == i as usize);
        }
        if let Some(val) = buf.peek() {
            assert!(val == 1);
        }

        assert!(buf.size() == 7);

        for i in 1..=7 {
            if let Some(val) = buf.remove() {
                assert!(val == i);
            }
            assert!(buf.size() == (7-i) as usize)
        }

        assert!(buf.is_empty());
    }

    #[test]
    fn circularity() {
        let mut buf = RingBuffer::<i32>::new(7);

        for i in 1..=8 {
            buf.add(i);
            if i < 8 {
                assert!(buf.size() == i as usize);
            }
        }
        assert!(buf.size() == 7);

        if let Some(val) = buf.peek() {
            assert!(val == 2);
        }

        buf.add(9);
        assert!(buf.size() == 7);

        if let Some(val) = buf.peek() {
            assert!(val == 3);
        }

        for i in 3..=9 {
            if let Some(val) = buf.remove() {
                assert!(val == i);
            }
            assert!(buf.size() == (9-i) as usize);
        }

        assert!(buf.is_empty());
        assert!(buf.size() == 0);

        if buf.remove().is_some() {
            panic!();
        }
    }

    #[test]
    fn new_buf_from_slice() {
        let mut buf = RingBuffer::<i32>::new_from_slice(&[9,4,1,5,6]);
        if let Some(val) = buf.peek() {
            assert!(val == 9);
        }
        assert!(buf.size() == 5);

        buf.add(10);
        assert!(buf.size() == 5);
        if let Some(val) = buf.peek() {
            assert!(val == 4);
        }

        if let Some(val) = buf.remove() {
            assert!(val == 4);
        }
        assert!(buf.size() == 4);
    }

    #[test]
    fn add_from_vec_to_slice() {
        let mut buf = RingBuffer::<i32>::new_from_slice(&[9,4,1,5,6]);
        buf.add_from_slice(&[1,2,3,4]);
        assert!(buf.size() == 5);
        if let Some(val) = buf.peek() {
            assert!(val == 6);
        }
    }
}