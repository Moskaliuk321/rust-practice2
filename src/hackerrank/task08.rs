/// Розв'язок задачі HackerRank: Breaking the Records
/// Практична робота №8
#[allow(dead_code)]
pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    // Проходимо по всіх матчах, починаючи з другого
    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            max_count += 1;
        } else if score < lowest {
            lowest = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_example() {
        // Приклад з умови: 9 матчів
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        // Максимум оновлювався 2 рази, мінімум — 4 рази
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }
}