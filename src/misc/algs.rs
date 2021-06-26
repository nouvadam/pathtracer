//! Various algorithms

/// Hoare algorithm, aka quickselect

pub fn quickselect<T>(v: &mut [T], k: usize) -> T
where
    T: Ord + Copy,
{
    bquickselect(v, 0, v.len() - 1, k)
}

fn bquickselect<T>(array: &mut [T], left_bound: usize, right_bound: usize, pos: usize) -> T
where
    T: Ord + Copy,
{
    if left_bound == right_bound {
        return array[left_bound];
    }

    let middle = partition(array, left_bound, right_bound);

    use std::cmp::Ordering;
    match pos.cmp(&middle) {
        Ordering::Greater => bquickselect(array, middle + 1, right_bound, pos),
        Ordering::Less => bquickselect(array, left_bound, middle - 1, pos),
        Ordering::Equal => array[pos],
    }
}

fn partition<T>(array: &mut [T], left_bound: usize, right_bound: usize) -> usize
where
    T: Ord + Copy,
{
    let middle = array[right_bound];
    let mut index = left_bound;

    for j in left_bound..right_bound {
        if array[j] <= middle {
            array.swap(index, j);
            index += 1;
        }
    }

    array.swap(index, right_bound);
    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::*;

    #[test]
    fn partition_test() {
        let mut arr = [9, 1, 8, 2, 7, 3, 6, 4, 5];
        partition(&mut arr, 0, 8);
        assert_eq!(arr, [1, 2, 3, 4, 5, 8, 6, 9, 7]);
    }

    #[test]
    fn random_partition_test() {
        let mut rng = rand::thread_rng();
        let mut array: Vec<u16> = (0..10).map(|_| rng.gen_range(0, 10)).collect();
        //println!("{:?}", array);
        let len = array.len() - 1;

        let mid = partition(&mut array, 0, len);
        //println!("{:?} : {}", array, mid);
        assert!(array_check(&array, mid));
    }

    fn array_check<T>(array: &[T], mid: usize) -> bool
    where
        T: Ord + Copy,
    {
        for i in 0..mid {
            if array[i] > array[mid] {
                return false;
            }
        }
        true
    }

    #[test]
    fn quickselect_test() {
        let mut rng = rand::thread_rng();
        let mut array: Vec<u16> = (0..9).map(|_| rng.gen_range(0, 9)).collect();
        println!("{:?}", array);
        let mut s = array.clone();
        s.sort();
        println!("{:?}", s);

        let middle = bquickselect(&mut array.clone(), 0, 8, 4);

        array.sort();

        assert_eq!(middle, array[4]);
    }
}
