/*
The permutations problem involves finding all possible permutations
of a given collection of distinct integers. For instance, given [1, 2, 3],
the goal is to generate permutations like
 [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], and [3, 2, 1].
 This implementation uses a backtracking algorithm to generate all possible permutations.
*/

pub fn permute(nums: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    backtrace(0, &mut result, &mut nums.clone());
    result
}

fn backtrace(depth: usize, result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>) {
    if depth == current.len() {
        result.push(current.clone());
    }

    for i in depth..current.len() {
        current.swap(depth, i);
        backtrace(depth + 1, result, current);
        current.swap(depth, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let result = permute(&(1..4).collect());
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![1, 2, 3]));
        assert!(result.contains(&vec![1, 3, 2]));
        assert!(result.contains(&vec![2, 1, 3]));
        assert!(result.contains(&vec![2, 3, 1]));
        assert!(result.contains(&vec![3, 1, 2]));
        assert!(result.contains(&vec![3, 2, 1]));
    }
}
