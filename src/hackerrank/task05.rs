/// Розв'язок задачі HackerRank: Apple and Orange
/// Практична робота №5
pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    // Рахуємо яблука, що впали на будинок
    let apple_count = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    // Рахуємо апельсини, що впали на будинок
    let orange_count = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apple_and_orange_example() {
        // Приклад з умови HackerRank
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let (res_apples, res_oranges) = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
        
        assert_eq!(res_apples, 1);
        assert_eq!(res_oranges, 1);
    }
}