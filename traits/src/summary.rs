pub trait Summary {
    fn summarise(&self) -> String {
        String::from("Read more...")
    }
}
