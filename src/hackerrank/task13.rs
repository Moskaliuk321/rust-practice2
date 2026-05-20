/// Розв'язок задачі HackerRank: Divisible Sum Pairs
/// Практична робота №13
#[allow(dead_code)]
pub fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let usize_n = n as usize;

    // Зовнішній цикл фіксує перший елемент пари
    for i in 0..usize_n {
        // Внутрішній цикл шукає другий елемент строго праворуч від першого
        for j in (i + 1)..usize_n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs_example() {
        let n = 6;
        let k = 3;
        let ar = vec![1, 3, 2, 6, 1, 2];
        // Для цього прикладу є рівно 5 пар
        assert_eq!(divisible_sum_pairs(n, k, &ar), 5);
    }
}