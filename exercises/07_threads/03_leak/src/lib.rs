// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread::{self, spawn};

pub fn sum(v: Vec<i32>) -> i32 {
    let afetrV = v.leak();
    let (v1,v2) = afetrV.split_at(afetrV.len()/2);
    // v1.to_vec().leak();
    // v2.to_vec().leak();
    let h1 = spawn(move||{
        v1.iter().sum::<i32>()
    });
    let h2 = spawn(move||{
        v2.iter().sum::<i32>()
    });

    h1.join().unwrap() + h2.join().unwrap()
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
