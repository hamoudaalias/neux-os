pub mod config {
    pub struct Config {
        pub model: String,
        pub batch_size: usize,
        pub index_path: String,
    }
    
    impl Default for Config {
        fn default() -> Self {
            Self {
                model: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
                batch_size: 4,
                index_path: ".neux/index".to_string(),
            }
        }
    }
}