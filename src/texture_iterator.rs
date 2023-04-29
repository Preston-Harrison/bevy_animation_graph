pub struct TextureIterator {
    offset: usize,
    length: usize,
    current: usize,
}

impl TextureIterator {
    pub fn new(first: usize, last: usize) -> Self {
        assert!(first <= last, "last frame cannot be before first");
        TextureIterator {
            offset: first,
            length: 1 + last - first,
            current: 0,
        }
    }

    pub fn next_few(&mut self, amount: usize) -> usize {
        self.current = (self.current + amount) % self.length;
        self.current()
    }
    pub fn next(&mut self) -> usize {
        self.next_few(1)
    }

    pub fn current(&self) -> usize {
        self.offset + self.current
    }

    pub fn is_last_frame(&self) -> bool {
        self.length == self.current + 1
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iteration() {
        let mut ti = TextureIterator::new(0, 3);
        assert_eq!(ti.current(), 0);
        assert_eq!(ti.length(), 4);
        assert_eq!(ti.next(), 1);
        ti.next();
        ti.next();
        assert!(ti.is_last_frame());
        assert_eq!(ti.current(), 3);
        assert_eq!(ti.next(), 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_creation() {
        TextureIterator::new(1, 0);
    }

    #[test]
    fn test_no_panic_one_frame() {
        TextureIterator::new(0, 0);
    }
}
