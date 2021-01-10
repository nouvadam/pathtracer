//! Zipping multiple iterators together

/// Struct containing Vector of iterators
#[derive(Clone)]
pub struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate through iterators in Vector, and for each one calls Next method, then collect all results from the Next method.
        self.0.iter_mut().map(Iterator::next).collect()
    }
}
/// Multizip functionality
pub trait IntoMultizip<T>: Sized {
    /// Returns struct that contains vector of iterators.
    fn multizip(self) -> Multizip<T>;
}

impl<I: Iterator<Item = Element>, Element: Iterator> IntoMultizip<Element> for I {
    fn multizip(self) -> Multizip<Element> {
        Multizip(self.collect())
    }
}
