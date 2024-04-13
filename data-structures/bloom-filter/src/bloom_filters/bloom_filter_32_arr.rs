use crate::bloom_filter::BloomFilter;

#[derive(Default)]
pub struct BloomFilter32 {
    bits: [bool; 32],
}

impl std::fmt::Debug for BloomFilter32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits: String = self
            .bits
            .iter()
            .map(|&x| if x { "1 " } else { "0 " })
            .collect();
        write!(f, "{:?}", bits)
    }
}

impl BloomFilter32 {
    fn additive_hasher(key: &str, seed: usize) -> usize {
        key.chars().fold(0, |acc, ch| -> usize {
            (acc + seed + (ch as usize % 32)) % 32 // modulo math return 0 - 31
        })
    }
}


impl BloomFilter for BloomFilter32 {
    fn insert(&mut self, key: &str) {
        let hash_a = Self::additive_hasher(key, 0);
        let hash_b = Self::additive_hasher(key, 1);

        self.bits[hash_a] = true;
        self.bits[hash_b] = true;
    }
    
    fn contains(&self, key: &str) -> bool {
        let hash_a = Self::additive_hasher(key, 0);
        let hash_b = Self::additive_hasher(key, 1);

        self.bits[hash_a] && self.bits[hash_b] 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_zeros() {
        let bl = BloomFilter32::default();

        bl.bits.iter().for_each(|bit| assert!(!bit));
    }

    #[test]
    fn test_additive_hasher_empty_string() {
        assert_eq!(BloomFilter32::additive_hasher("", 0), 0);
    }

    #[test]
    fn test_additive_hasher_seed_sensitive() {
        assert_ne!(
            BloomFilter32::additive_hasher("ad", 0),
            BloomFilter32::additive_hasher("ad", 1)
        );
    }
}