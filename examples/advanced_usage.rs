use bloomfilter_rs::BloomFilter;
use std::collections::HashSet;

fn main() {
    // Create a BloomFilter with custom parameters
    let mut filter = BloomFilter::new(10000, 4).unwrap();

    // Add multiple items at once
    let items = vec!["rust", "golang", "python", "java", "javascript"];
    for item in &items {
        filter.insert(item);
    }

    // Test for false positives
    let mut false_positives = 0;
    let test_size = 1000;
    let test_items: Vec<String> = (0..test_size)
        .map(|i| format!("test_item_{}", i))
        .collect();

    for item in &test_items {
        if filter.check(item) {
            false_positives += 1;
        }
    }

    println!(
        "False positive rate (experimental): {:.4}",
        false_positives as f64 / test_size as f64
    );

    // Compare with HashSet for verification
    let mut set = HashSet::new();
    for item in &items {
        set.insert(*item);
    }

    println!("\nVerification with HashSet:");
    for item in &items {
        let in_filter = filter.check(item);
        let in_set = set.contains(*item);
        println!(
            "Item '{}': BloomFilter={}, HashSet={}",
            item, in_filter, in_set
        );
    }

    // Performance comparison
    println!("\nPerformance Test:");
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        filter.check(&"rust");
    }
    println!("BloomFilter lookup time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    for _ in 0..10000 {
        set.contains("rust");
    }
    println!("HashSet lookup time: {:?}", start.elapsed());
}
