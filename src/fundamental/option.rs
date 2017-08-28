pub trait AsOption {
    fn as_option<T, F>(self, f: F) -> Option<T> where F: Fn() -> T;
}

impl AsOption for bool {
    fn as_option<T, F>(self, f: F) -> Option<T> where F: Fn() -> T {
        if self {
            Some(f())
        } else {
            None
        }
    }
}
