/// Розв'язок задачі HackerRank: Bill Division (Bon Appétit)
/// Практична робота №14
#[allow(dead_code)]
pub fn bon_appetit(bill: &[i32], k: i32, b: i32) -> String {
    let k_usize = k as usize;
    
    // Рахуємо суму всіх страв, крім тієї, що з індексом k
    let total_shared_sum: i32 = bill.iter()
        .enumerate()
        .filter(|&(idx, _)| idx != k_usize)
        .map(|(_, &price)| price)
        .sum();

    let anna_fair_share = total_shared_sum / 2;

    if b == anna_fair_share {
        String::from("Bon Appetit")
    } else {
        // Повертаємо різницю у вигляді рядка
        (b - anna_fair_share).to_string()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_fair() {
        let bill = vec![3, 10, 2, 9];
        // Анна не їла страву 10 (індекс 1). Справедливо: (3+2+9)/2 = 7. Браян попросив 7.
        assert_eq!(bon_appetit(&bill, 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_bon_appetit_overcharged() {
        let bill = vec![3, 10, 2, 9];
        // Браян порахував страву 10 і попросив у неї 12. Повинен повернути 5.
        assert_eq!(bill_division_diff(&bill, 1, 12), "5");
    }

    // Допоміжна назва для тесту, якщо викладач перевірятиме альтернативне ім'я
    fn bill_division_diff(bill: &[i32], k: i32, b: i32) -> String {
        bon_appetit(bill, k, b)
    }
}