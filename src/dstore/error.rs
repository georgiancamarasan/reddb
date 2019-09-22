quick_error! {
    #[derive(Debug)]
    pub enum DStoreError {
        Io(err: ::std::io::Error) {
            description(err.description())
            display("IO error: {}", err)
            cause(err)
            from()
        }
         Deserialize(err: serde_json::Error) {
            from()
        }
        Poison {}
        NotFound {}
    }
}

impl<T> From<::std::sync::PoisonError<T>> for DStoreError {
    fn from(_: ::std::sync::PoisonError<T>) -> DStoreError {
        DStoreError::Poison
    }
}