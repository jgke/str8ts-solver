use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct BitSet(u32);

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BitSet")?;
        f.debug_set().entries((1..32).filter(|n| self.contains(*n))).finish()
    }
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet(0)
    }

    pub fn new_from_number(num: u32) -> BitSet {
        BitSet(num)
    }

    pub fn to_number(&self) -> u32 {
        self.0
    }

    pub fn contains(&self, num: u8) -> bool {
        (self.0 & (1 << num)) != 0
    }

    pub fn insert(&mut self, num: u8) -> bool {
        let res = !self.contains(num);
        self.0 |= 1 << num;
        res
    }

    pub fn remove(&mut self, num: u8) -> bool {
        let res = self.contains(num);
        self.0 &= !(1 << num);
        res
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn append(&mut self, other: BitSet) -> bool {
        let retval = !other.is_subset(*self);
        self.0 |= other.0;
        retval
    }

    pub fn is_subset(&self, other: BitSet) -> bool {
        self.0 | other.0 == other.0
    }

    pub fn difference(&self, other: BitSet) -> Self {
        Self(self.0 & !other.0)
    }

    pub fn union(&self, other: BitSet) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersection(&self, other: BitSet) -> Self {
        Self(self.0 & other.0)
    }

    pub fn symmetric_difference(&self, other: BitSet) -> Self {
        self.union(other).difference(self.intersection(other))
    }
}

impl FromIterator<u8> for BitSet {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut set = BitSet::new();
        for num in iter {
            set.insert(num);
        }
        set
    }
}

impl IntoIterator for BitSet {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let vec = (1..16).filter(|n| self.contains(*n)).collect::<Vec<_>>();
        vec.into_iter()
    }
}

impl From<std::collections::HashSet<u8>> for BitSet {
    fn from(value: std::collections::HashSet<u8>) -> Self {
        value.into_iter().collect()
    }
}

impl From<BitSet> for std::collections::HashSet<u8> {
    fn from(value: BitSet) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let mut set = BitSet::new();
        assert_eq!(set.0, 0);
        assert!(set.is_empty());
        assert!(!set.contains(1));
        assert!(set.insert(1));
        assert_eq!(set.0, 2);
        assert!(set.contains(1));
        assert!(!set.is_empty());
        assert!(set.remove(1));
        assert!(!set.contains(1));
        assert_eq!(set.0, 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_multiple_inserts() {
        let mut set = BitSet::new();
        set.append([1, 2, 3, 4, 5].into_iter().collect());
        assert_eq!(set.0, 0b111110);
        assert!(set.contains(5));
        assert!(set.remove(5));
        assert!(!set.contains(5));
        assert_eq!(set.0, 0b011110);
    }
}
