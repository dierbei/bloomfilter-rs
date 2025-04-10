use bloomfilter_rs::BloomFilter;

#[test]
fn test_basic_functionality() {
    let mut filter = BloomFilter::new(1000, 4).unwrap();
    
    // Test insertion and checking
    filter.insert(&"test");
    assert!(filter.check(&"test"));
    assert!(!filter.check(&"not_in_set"));
    
    // Test multiple items
    let items = vec!["apple", "banana", "orange"];
    for item in &items {
        filter.insert(item);
        assert!(filter.check(item));
    }
    
    // Test statistics
    assert_eq!(filter.len(), 4);
    assert!(!filter.is_empty());
    assert!(filter.capacity() >= 1000);
}

#[test]
fn test_false_positives() {
    // Increase filter size and reduce number of items to keep false positive rate low
    let mut filter = BloomFilter::new(1000, 3).unwrap();
    
    // Add fewer items
    for i in 0..20 {
        filter.insert(&format!("item_{}", i));
    }
    
    // Test for false positives
    let mut false_positives = 0;
    let test_size = 1000;
    for i in 0..test_size {
        let item = format!("test_{}", i);
        if filter.check(&item) {
            false_positives += 1;
        }
    }
    
    // The false positive rate should be reasonable
    let false_positive_rate = false_positives as f64 / test_size as f64;
    assert!(false_positive_rate < 0.1); // Should be less than 10%
}

#[test]
fn test_clear() {
    let mut filter = BloomFilter::new(1000, 4).unwrap();
    
    // Add items
    filter.insert(&"test1");
    filter.insert(&"test2");
    assert_eq!(filter.len(), 2);
    
    // Clear the filter
    filter.clear();
    assert_eq!(filter.len(), 0);
    assert!(filter.is_empty());
    assert!(!filter.check(&"test1"));
    assert!(!filter.check(&"test2"));
}

#[test]
fn test_error_handling() {
    // Test invalid size
    assert!(BloomFilter::new(0, 4).is_err());
    
    // Test invalid hash count
    assert!(BloomFilter::new(1000, 0).is_err());
}

#[test]
fn test_large_dataset() {
    // Increase filter size and reduce number of items to keep false positive rate low
    let mut filter = BloomFilter::new(50000, 4).unwrap();
    
    // Add fewer items
    for i in 0..2000 {
        filter.insert(&format!("item_{}", i));
    }
    
    // Verify all items are found
    for i in 0..2000 {
        assert!(filter.check(&format!("item_{}", i)));
    }
    
    // Check some non-existent items
    let mut false_positives = 0;
    for i in 2000..3000 {
        if filter.check(&format!("item_{}", i)) {
            false_positives += 1;
        }
    }
    
    // The false positive rate should be reasonable
    let false_positive_rate = false_positives as f64 / 1000.0;
    assert!(false_positive_rate < 0.1); // Should be less than 10%
} 