/// Розв'язок задачі HackerRank: Birthday Cake Candles
/// Практична робота №12
#[allow(dead_code)]
pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    // Знаходимо максимальну висоту серед свічок
    if let Some(&max_height) = candles.iter().max() {
        // Рахуємо, скільки свічок мають таку саму максимальну висоту
        candles.iter().filter(|&&h| h == max_height).count() as i32
    } else {
        0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_birthday_cake_candles_example() {
        let candles = vec![4, 4, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }
}