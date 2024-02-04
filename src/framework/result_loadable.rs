pub trait ResultLoadable<T> {
    fn create_from_result(result: &T) -> Self;
}
