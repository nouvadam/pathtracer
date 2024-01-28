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

impl<I, E> IntoMultizip<E> for I
where
    I: Iterator<Item = E>,
    E: Iterator,
{
    fn multizip(self) -> Multizip<E> {
        Multizip(self.collect())
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_multizip() {
        let first_vec = vec![1, 2, 3];
        let second_vec = vec![4, 5, 6];
        let third_vec = vec![7, 8, 9];

        let result_vec = vec![first_vec, second_vec, third_vec]
            .into_iter()
            .map(|element| element.into_iter())
            .multizip();

        let correct_vec = vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]];

        assert_equal(result_vec, correct_vec);
    }

    #[test]
    fn test_multizip2() {
        let first_vec = vec![1, 2];
        let second_vec = vec![4, 5, 6];
        let third_vec = vec![7, 8, 9, 10];

        let result_vec = vec![first_vec, second_vec, third_vec]
            .into_iter()
            .map(|element| element.into_iter())
            .multizip();

        let correct_vec = vec![[1, 4, 7], [2, 5, 8]];

        assert_equal(result_vec, correct_vec);
    }

    #[test]
    fn test_multizip3() {
        let first_vec = vec![1, 2, 3];
        let second_vec = vec![4, 5, 6];
        let third_vec = vec![];

        let result_vec = vec![first_vec, second_vec, third_vec]
            .into_iter()
            .map(|element| element.into_iter())
            .multizip();

        let correct_vec: Vec<[i32; 0]> = vec![];

        assert_equal(result_vec, correct_vec);
    }
}
