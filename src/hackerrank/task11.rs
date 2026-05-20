/// Розв'язок задачі HackerRank: Diagonal Difference
/// Практична робота №11
#[allow(dead_code)]
pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    // Повертаємо абсолютну різницю (модуль)
    (primary_sum - secondary_sum).abs()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_example() {
        let arr = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];
        assert_eq!(diagonal_difference(&arr), 2);
    }
}