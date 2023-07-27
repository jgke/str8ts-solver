#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct BitSet(u32);

impl BitSet {
    pub fn new() -> BitSet {
        BitSet(0)
    }

    pub fn contains(&self, num: u8) -> bool {
        (self.0 & (1 << num)) != 0
    }

    pub fn insert(&mut self, num: u8) -> bool {
        let res = !self.contains(num);
        self.0 |= 1 << num;
        res
    }

    pub fn remove(&mut self, num: u8) {
        self.0 &= !(1 << num)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn difference(&self, other: BitSet) -> Self {
        Self(self.0 & !other.0)
    }

    pub fn union(&self, other: BitSet) -> Self {
        Self(self.0 | other.0)
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
