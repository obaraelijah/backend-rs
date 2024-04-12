pub mod bloom_filter;
pub mod bloom_filters;

use bloom_filter::BloomFilter;
//use bloom_filters::bloom_filter_32_arr::BloomFilter32;

fn main() {
    // let mut bl = BloomFilter32::default();
}

fn bloomy(bl: &mut impl BloomFilter) {
    let keys = vec!["mango", "apple", "orange", "banana"];

    keys.iter().for_each(|&key| bl.insert(key));

    let mut test_keys = vec!["carrot", "radish", "vegetables", "onion"];
    test_keys.extend(keys);

    let results = test_keys.iter().map(|&key| bl.contains(key));
}
