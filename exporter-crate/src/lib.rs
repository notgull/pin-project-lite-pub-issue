// MIT/Apache2 License

pub use pin_project_lite;

#[macro_export]
macro_rules! wrapper {
    ($vis:vis) => {
        $crate::pin_project_lite::pin_project! {
            $vis struct Wrapper<T> {
                #[pin]
                pub inner: T,
            }
        }
    };
}
