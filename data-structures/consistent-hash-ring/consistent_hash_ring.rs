pub trait ConsitentHashRing {
    type ConsumerInfo;

    fn add_consumer(&mut self, key: &str, data: Self::ConsumerInfo);
    fn remove_consumer(&mut self, key: &str);

    fn get_consumer(&self, key: &str) -> Option<&Self::ConsumerInfo>;
}