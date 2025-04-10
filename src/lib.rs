use bitvec::prelude::*;
use md5::{Md5, Digest};
use sha2::Sha256;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod memory_test;

/// A Bloom filter is a space-efficient probabilistic data structure that is used to test whether an element is a member of a set.
/// False positive matches are possible, but false negatives are not. In other words, a query returns either "possibly in set" or "definitely not in set".
///
/// # Examples
/// ```
/// use bloomfilter_rs::BloomFilter;
///
/// let mut bf = BloomFilter::new(1000, 3).unwrap();
/// bf.insert(&"test");
/// assert!(bf.check(&"test")); // true
/// assert!(!bf.check(&"not_in_set")); // false
/// ```
pub struct BloomFilter {
    /// Bit array representing the filter
    buckets: BitVec,
    /// List of hash algorithms to use
    hash_algorithms: Vec<HashAlgorithm>,
    /// Number of items inserted into the filter
    item_count: u64,
}

/// Hash algorithms supported by the Bloom filter
#[derive(Clone, Copy)]
enum HashAlgorithm {
    /// Rust's default hasher
    Default,
    /// MD5 hash algorithm
    MD5,
    /// SHA256 hash algorithm
    SHA256,
}

/// Errors that can occur when working with a Bloom filter
#[derive(Debug)]
pub enum BloomFilterError {
    /// The size of the filter must be greater than 0
    InvalidSize,
    /// The number of hash functions must be greater than 0
    InvalidHashCount,
    /// An overflow occurred during calculation
    Overflow,
}

/// Computes a hash value for the given object using the specified algorithm
///
/// # Arguments
/// * `obj` - The object to hash
/// * `algorithm` - The hash algorithm to use
///
/// # Returns
/// A 64-bit hash value
fn hash_with_algorithm<T>(obj: &T, algorithm: HashAlgorithm) -> u64
where
    T: Hash,
{
    match algorithm {
        HashAlgorithm::Default => {
            let mut hasher = DefaultHasher::new();
            obj.hash(&mut hasher);
            hasher.finish()
        }
        HashAlgorithm::MD5 => {
            let mut temp_hasher = DefaultHasher::new();
            obj.hash(&mut temp_hasher);
            let hash_bytes = temp_hasher.finish().to_le_bytes();
            let mut hasher = Md5::new();
            hasher.update(&hash_bytes);
            let result = hasher.finalize();
            result.iter().fold(0u64, |acc, &x| (acc << 8) | x as u64)
        }
        HashAlgorithm::SHA256 => {
            let mut temp_hasher = DefaultHasher::new();
            obj.hash(&mut temp_hasher);
            let hash_bytes = temp_hasher.finish().to_le_bytes();
            let mut hasher = Sha256::new();
            hasher.update(&hash_bytes);
            let result = hasher.finalize();
            result.iter().fold(0u64, |acc, &x| (acc << 8) | x as u64)
        }
    }
}

impl BloomFilter {
    /// Creates a new Bloom filter with the specified size and number of hash functions
    ///
    /// # Arguments
    /// * `size` - The size of the bit array (number of buckets)
    /// * `hash_count` - The number of hash functions to use
    ///
    /// # Returns
    /// A new Bloom filter or an error if the parameters are invalid
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let bf = BloomFilter::new(1000, 3).unwrap();
    /// ```
    pub fn new(size: usize, hash_count: usize) -> Result<BloomFilter, BloomFilterError> {
        if size == 0 {
            return Err(BloomFilterError::InvalidSize);
        }
        if hash_count == 0 {
            return Err(BloomFilterError::InvalidHashCount);
        }

        let buckets = bitvec![0; size];
        let hash_algorithms = vec![
            HashAlgorithm::Default,
            HashAlgorithm::MD5,
            HashAlgorithm::SHA256,
        ]
        .into_iter()
        .cycle()
        .take(hash_count)
        .collect();

        Ok(BloomFilter {
            buckets,
            hash_algorithms,
            item_count: 0,
        })
    }

    /// Inserts an element into the Bloom filter
    ///
    /// # Arguments
    /// * `word` - The element to insert
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let mut bf = BloomFilter::new(1000, 3).unwrap();
    /// bf.insert(&"test");
    /// ```
    pub fn insert<T>(&mut self, word: &T)
    where
        T: Hash,
    {
        for &algorithm in &self.hash_algorithms {
            let i: usize = self.bloom_hash(word, algorithm);
            self.buckets.set(i, true);
        }
        self.item_count += 1;
    }

    /// Checks if an element is possibly in the set
    ///
    /// # Arguments
    /// * `word` - The element to check
    ///
    /// # Returns
    /// `true` if the element is possibly in the set, `false` if it is definitely not in the set
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let mut bf = BloomFilter::new(1000, 3).unwrap();
    /// bf.insert(&"test");
    /// assert!(bf.check(&"test")); // true
    /// ```
    pub fn check<T>(&self, word: &T) -> bool
    where
        T: Hash,
    {
        for &algorithm in &self.hash_algorithms {
            let i: usize = self.bloom_hash(word, algorithm);
            if !self.buckets[i] {
                return false;
            }
        }
        true
    }

    /// Calculates the probability of a false positive
    ///
    /// # Returns
    /// The probability of a false positive as a float between 0 and 1
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let bf = BloomFilter::new(1000, 3).unwrap();
    /// let error_rate = bf.error_chance();
    /// println!("False positive probability: {}", error_rate);
    /// ```
    pub fn error_chance(&self) -> f32 {
        let numerator = (self.hash_algorithms.len() as u64 * self.item_count) as f32;
        let denominator = self.buckets.len() as f32;
        let e_exponent = (-1.0 * numerator) / denominator;
        let chance: f32 = (1.0 - e_exponent.exp()).powf(self.hash_algorithms.len() as f32);
        chance
    }

    /// Computes a hash value for the given object and maps it to a bucket index
    ///
    /// # Arguments
    /// * `word` - The object to hash
    /// * `algorithm` - The hash algorithm to use
    ///
    /// # Returns
    /// A bucket index
    fn bloom_hash<T>(&self, word: &T, algorithm: HashAlgorithm) -> usize
    where
        T: Hash,
    {
        let the_hash: usize = hash_with_algorithm(word, algorithm) as usize;
        the_hash % self.buckets.len()
    }

    /// Clears all elements from the Bloom filter
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let mut bf = BloomFilter::new(1000, 3).unwrap();
    /// bf.insert(&"test");
    /// assert_eq!(bf.len(), 1);
    /// bf.clear();
    /// assert_eq!(bf.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.buckets.fill(false);
        self.item_count = 0;
    }

    /// Returns the capacity of the Bloom filter (number of buckets)
    ///
    /// # Returns
    /// The number of buckets in the filter
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let bf = BloomFilter::new(1000, 3).unwrap();
    /// assert_eq!(bf.capacity(), 1000);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    /// Returns the number of elements inserted into the Bloom filter
    ///
    /// # Returns
    /// The number of elements inserted
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let mut bf = BloomFilter::new(1000, 3).unwrap();
    /// bf.insert(&"test");
    /// assert_eq!(bf.len(), 1);
    /// ```
    pub fn len(&self) -> u64 {
        self.item_count
    }

    /// Checks if the Bloom filter is empty
    ///
    /// # Returns
    /// `true` if no elements have been inserted, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// use bloomfilter_rs::BloomFilter;
    ///
    /// let bf = BloomFilter::new(1000, 3).unwrap();
    /// assert!(bf.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_correct_size() {
        let bf = BloomFilter::new(10, 1).unwrap();
        assert!(bf.buckets.len() == 10);
    }

    #[test]
    fn insert_and_check_str() {
        let mut bf = BloomFilter::new(100, 4).unwrap();
        bf.insert(&"coffee");

        assert!(bf.check(&"coffee") == true);
        assert!(bf.check(&"pancakes") == false);
    }

    #[test]
    fn insert_and_check_other() {
        let mut bf = BloomFilter::new(100, 4).unwrap();
        let the_answer = 42;
        let the_devil = 666;

        bf.insert(&the_answer);

        assert!(bf.check(&the_answer) == true);
        assert!(bf.check(&the_devil) == false);
    }

    #[test]
    fn insert_and_increment_item_count() {
        let mut bf = BloomFilter::new(100, 4).unwrap();
        assert!(bf.item_count == 0);

        bf.insert(&"coffee");

        assert!(bf.item_count == 1);

        bf.insert(&"ham");

        assert!(bf.item_count == 2);
    }

    #[test]
    fn error_chance() {
        let mut bf = BloomFilter::new(100, 4).unwrap();

        bf.insert(&"coffee");

        assert!(bf.error_chance().floor() == 0.0);
    }

    #[test]
    fn test_clear() {
        let mut bf = BloomFilter::new(100, 4).unwrap();
        bf.insert(&"test");
        assert_eq!(bf.len(), 1);
        bf.clear();
        assert_eq!(bf.len(), 0);
        assert!(bf.is_empty());
    }

    #[test]
    fn test_capacity() {
        let bf = BloomFilter::new(100, 4).unwrap();
        assert_eq!(bf.capacity(), 100);
    }

    #[test]
    fn test_invalid_params() {
        assert!(BloomFilter::new(0, 1).is_err());
        assert!(BloomFilter::new(1, 0).is_err());
    }
}
