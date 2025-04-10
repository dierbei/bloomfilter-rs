use bloomfilter_rs::BloomFilter;

fn main() {
    // Create a new BloomFilter with expected size and number of hash functions
    let mut filter = BloomFilter::new(1000, 4).unwrap();

    // Add some items
    filter.insert(&"apple");
    filter.insert(&"banana");
    filter.insert(&"orange");

    // Check if items exist
    println!("Contains 'apple': {}", filter.check(&"apple")); // true
    println!("Contains 'banana': {}", filter.check(&"banana")); // true
    println!("Contains 'orange': {}", filter.check(&"orange")); // true
    println!("Contains 'grape': {}", filter.check(&"grape")); // false (probably)

    // Get filter statistics
    println!("\nFilter Statistics:");
    println!("Error chance: {}", filter.error_chance());
    println!("Capacity: {}", filter.capacity());
    println!("Length: {}", filter.len());
    println!("Is empty: {}", filter.is_empty());
}
