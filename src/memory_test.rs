use bitvec::prelude::*;
use std::mem;

#[allow(dead_code)]
pub fn compare_memory_usage() {
    let sizes = [1000, 10000, 100000];

    for &size in &sizes {
        let vec_bool = vec![false; size];
        let bit_vec = bitvec![0; size];

        let vec_bool_size = size * mem::size_of::<bool>();
        let bit_vec_size = (size + 7) / 8;

        println!("\nArray size: {}", size);
        println!("Vec<bool> size: {} bytes", vec_bool_size);
        println!("BitVec size: {} bytes", bit_vec_size);
        println!(
            "Memory savings: {:.2}x",
            vec_bool_size as f64 / bit_vec_size as f64
        );

        assert_eq!(vec_bool.len(), bit_vec.len());
        assert_eq!(vec_bool[0], bit_vec[0]);
        assert_eq!(vec_bool[size - 1], bit_vec[size - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_usage() {
        compare_memory_usage();
    }
}
