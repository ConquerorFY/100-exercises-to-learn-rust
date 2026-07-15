// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    thread::scope(|scope| {
        let thread1 = scope.spawn(|| (&v[..mid]).iter().sum::<i32>());
        let thread2 = scope.spawn(|| (&v[mid..]).iter().sum::<i32>());
        thread1.join().unwrap() + thread2.join().unwrap()
    })

    // // The compiler knows v[..mid] is a slice,
    // // so it automatically borrows it for the .iter() call.
    // let thread1 = scope.spawn(|| v[..mid].iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
