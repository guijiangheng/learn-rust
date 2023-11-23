/*
    In this problem, we want to determine all possible combinations of k
    numbers out of 1 ... n. We use backtracking to solve this problem.
    Time complexity: O(C(n,k)) which is O(n choose k) = O((n!/(k! * (n - k)!)))

    generate_all_combinations(n=4, k=2) => [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
*/
pub fn generate_all_combinations(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    create_all_state(1, 1, n, k, &mut vec![], &mut result);
    result
}

fn create_all_state(
    level: i32,
    start: i32,
    total_number: i32,
    k: i32,
    current_list: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if level > k {
        result.push(current_list.clone());
        return;
    }

    for i in start..=total_number - k + level {
        current_list.push(i);
        create_all_state(level + 1, i + 1, total_number, k, current_list, result);
        current_list.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        assert_eq!(
            generate_all_combinations(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        )
    }

    #[test]
    fn test_empty() {
        assert_eq!(generate_all_combinations(4, 0), vec![vec![]]);
    }
}
